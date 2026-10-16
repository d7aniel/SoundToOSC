import React, { useEffect, useRef } from 'react';
import { TelemetryPayload } from '../types';
import { useLanguage } from '../i18n';

interface MasterVisualizerProps {
  telemetry: TelemetryPayload | null;
  isRunning: boolean;
}

export const MasterVisualizer: React.FC<MasterVisualizerProps> = ({ telemetry, isRunning }) => {
  const { t } = useLanguage();
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const telemetryRef = useRef<TelemetryPayload | null>(telemetry);
  telemetryRef.current = telemetry;
  const tRef = useRef(t);
  tRef.current = t;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    let animId: number;

    const render = () => {
      const width = canvas.clientWidth;
      const height = canvas.clientHeight;

      const dpr = window.devicePixelRatio || 1;
      if (canvas.width !== width * dpr || canvas.height !== height * dpr) {
        canvas.width = width * dpr;
        canvas.height = height * dpr;
      }

      ctx.save();
      ctx.scale(dpr, dpr);

      // Background
      ctx.fillStyle = '#0f141c';
      ctx.fillRect(0, 0, width, height);

      // Subtle grid lines
      ctx.strokeStyle = 'rgba(255, 255, 255, 0.04)';
      ctx.lineWidth = 1;
      const midY = height * 0.45;

      ctx.beginPath();
      ctx.moveTo(0, midY);
      ctx.lineTo(width, midY);
      ctx.stroke();

      const data = telemetryRef.current;

      if (isRunning && data && data.waveform && data.waveform.length > 0) {
        // --- 1. Draw Waveform (Oscilloscope) on Top Half ---
        const wave = data.waveform;
        const waveHeight = height * 0.4;
        const waveCenter = height * 0.22;

        ctx.strokeStyle = '#00ffc4';
        ctx.lineWidth = 2;
        ctx.shadowColor = 'rgba(0, 255, 196, 0.6)';
        ctx.shadowBlur = 8;
        ctx.beginPath();

        const stepX = width / (wave.length - 1);
        for (let i = 0; i < wave.length; i++) {
          const x = i * stepX;
          const y = waveCenter - wave[i] * waveHeight;
          if (i === 0) ctx.moveTo(x, y);
          else ctx.lineTo(x, y);
        }
        ctx.stroke();
        ctx.shadowBlur = 0; // reset

        // --- 2. Draw FFT Spectrum on Bottom Half ---
        const spectrum = data.spectrum || [];
        const specHeight = height * 0.42;
        const barCount = spectrum.length;

        if (barCount > 0) {
          const barWidth = width / barCount;
          const gap = 1.5;

          for (let i = 0; i < barCount; i++) {
            const val = Math.max(0, Math.min(1, spectrum[i]));
            const barH = val * specHeight;
            const x = i * barWidth;
            const y = height - barH;

            // Gradient: Bass (cyan/teal) -> Mid (cyan/blue) -> Treble (magenta/pink)
            const hue = 160 + (i / barCount) * 140; // 160 (cyan) to 300 (magenta)
            ctx.fillStyle = `hsl(${hue}, 90%, ${45 + val * 25}%)`;

            ctx.fillRect(x + gap * 0.5, y, barWidth - gap, barH);
          }
        }

        // --- 3. Stereo Position Balance Gauge ---
        const pos = data.values && data.values['stereo_pos'] ? (data.values['stereo_pos'][0] ?? 0) : 0;
        const clampedPos = Math.max(-1, Math.min(1, pos));
        const gaugeW = Math.min(200, Math.max(120, width * 0.28));
        const centerX = width / 2;
        const gaugeY = midY;

        // Track bar
        ctx.fillStyle = 'rgba(255, 255, 255, 0.08)';
        ctx.fillRect(centerX - gaugeW / 2, gaugeY - 2, gaugeW, 4);

        // Center notch
        ctx.fillStyle = 'rgba(255, 255, 255, 0.35)';
        ctx.fillRect(centerX - 1, gaugeY - 5, 2, 10);

        // L / R Markers
        ctx.font = '10px monospace';
        ctx.fillStyle = 'rgba(0, 255, 196, 0.75)';
        ctx.textAlign = 'right';
        ctx.fillText('L', centerX - gaugeW / 2 - 6, gaugeY + 3);

        ctx.fillStyle = 'rgba(168, 85, 247, 0.75)';
        ctx.textAlign = 'left';
        ctx.fillText('R', centerX + gaugeW / 2 + 6, gaugeY + 3);

        // Active deflection fill
        if (clampedPos < 0) {
          ctx.fillStyle = 'rgba(0, 255, 196, 0.65)';
          ctx.fillRect(centerX + clampedPos * (gaugeW / 2), gaugeY - 2, -clampedPos * (gaugeW / 2), 4);
        } else if (clampedPos > 0) {
          ctx.fillStyle = 'rgba(168, 85, 247, 0.65)';
          ctx.fillRect(centerX, gaugeY - 2, clampedPos * (gaugeW / 2), 4);
        }

        // Puck
        const puckX = centerX + clampedPos * (gaugeW / 2);
        ctx.shadowBlur = 8;
        ctx.shadowColor = clampedPos < 0 ? '#00ffc4' : clampedPos > 0 ? '#a855f7' : '#ffffff';
        ctx.fillStyle = clampedPos < 0 ? '#00ffc4' : clampedPos > 0 ? '#c084fc' : '#ffffff';
        ctx.beginPath();
        ctx.arc(puckX, gaugeY, 4, 0, Math.PI * 2);
        ctx.fill();
        ctx.shadowBlur = 0;

        // Position readout text above gauge
        ctx.font = '9px monospace';
        ctx.fillStyle = 'rgba(255, 255, 255, 0.45)';
        ctx.textAlign = 'center';
        const posText = Math.abs(clampedPos) < 0.02
          ? tRef.current.stereoCenter
          : `${clampedPos < 0 ? 'L' : 'R'} ${Math.abs(clampedPos).toFixed(2)}`;
        ctx.fillText(`${tRef.current.stereoPositionGauge}: ${posText}`, centerX, gaugeY - 9);
      } else {
        // Idle display
        ctx.fillStyle = 'rgba(255, 255, 255, 0.15)';
        ctx.font = '13px monospace';
        ctx.textAlign = 'center';
        ctx.fillText(
          isRunning ? tRef.current.visualizerListening : tRef.current.visualizerStopped,
          width / 2,
          height / 2
        );
      }

      // Frequency Band Label Overlays
      ctx.fillStyle = 'rgba(255, 255, 255, 0.3)';
      ctx.font = '10px monospace';
      ctx.textAlign = 'left';
      ctx.fillText(tRef.current.oscilloscope, 12, 16);
      ctx.fillText(tRef.current.fftSpectrum, 12, height * 0.52 - 6);

      ctx.textAlign = 'right';
      ctx.fillText(tRef.current.bandSub, width * 0.25 - 8, height - 6);
      ctx.fillText(tRef.current.bandLow, width * 0.5 - 8, height - 6);
      ctx.fillText(tRef.current.bandMid, width * 0.75 - 8, height - 6);
      ctx.fillText(tRef.current.bandHigh, width - 8, height - 6);

      ctx.restore();
      animId = requestAnimationFrame(render);
    };

    animId = requestAnimationFrame(render);
    return () => cancelAnimationFrame(animId);
  }, [isRunning]);


  return (
    <div className="master-visualizer-container">
      <canvas ref={canvasRef} className="visualizer-canvas" />
    </div>
  );
};
