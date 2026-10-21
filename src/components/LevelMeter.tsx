import { Icon, IconName } from "./Icon";

interface Props {
  icon: IconName;
  label: string;
  /** Pico de 0 a 1. */
  level: number;
  /** Fonte sem stream (ex.: dispositivo desconectado). */
  offline: boolean;
}

/** Escala perceptual: fala normal fica no meio da barra, nao colada no inicio. */
function toPercent(level: number) {
  if (level <= 0) return 0;
  const db = 20 * Math.log10(level);
  return Math.max(0, Math.min(100, ((db + 50) / 50) * 100));
}

export function LevelMeter({ icon, label, level, offline }: Props) {
  const percent = offline ? 0 : toPercent(level);
  return (
    <div className={`meter ${offline ? "is-offline" : ""}`}>
      <Icon name={icon} size={16} />
      <span className="meter-label">{label}</span>
      <span className="meter-track" role="meter" aria-label={label} aria-valuenow={Math.round(percent)} aria-valuemin={0} aria-valuemax={100}>
        <span className="meter-fill" style={{ width: `${percent}%` }} />
      </span>
      {offline && <span className="meter-status">sem sinal</span>}
    </div>
  );
}
