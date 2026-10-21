interface Props {
  enabled: boolean;
  disabled: boolean;
  onToggle: () => void;
}

export function ScreenToggle({ enabled, disabled, onToggle }: Props) {
  return (
    <label className="screen-toggle">
      <input type="checkbox" checked={enabled} disabled={disabled} onChange={onToggle} />
      Gravar tela
    </label>
  );
}
