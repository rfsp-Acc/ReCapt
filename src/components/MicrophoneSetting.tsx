import { InputDevice } from "../lib/tauri";

interface Props {
  devices: InputDevice[];
  /** `null` segue o padrao do sistema. */
  selected: string | null;
  onSelect: (id: string | null) => void;
  /** Recarrega a lista: dispositivos Bluetooth podem ter sido conectados depois. */
  onRefresh: () => void;
}

const DEFAULT = "__default__";

export function MicrophoneSetting({ devices, selected, onSelect, onRefresh }: Props) {
  const current = devices.find((d) => d.isDefault);
  const missing = selected !== null && !devices.some((d) => d.id === selected);

  return (
    <label className="language-setting">
      <span className="output-dir-label">Microfone</span>
      <select
        value={selected ?? DEFAULT}
        onFocus={onRefresh}
        onChange={(e) => onSelect(e.target.value === DEFAULT ? null : e.target.value)}
      >
        <option value={DEFAULT}>Padrão do sistema{current ? ` (${current.name})` : ""}</option>
        {devices.map((d) => (
          <option key={d.id} value={d.id}>
            {d.name}
          </option>
        ))}
        {missing && <option value={selected}>Dispositivo desconectado</option>}
      </select>
      <span className="hint">
        {missing
          ? "O microfone escolhido não está conectado; o padrão do sistema será usado."
          : "Com fone Bluetooth, o microfone do computador mantém o áudio do fone em alta qualidade."}
      </span>
    </label>
  );
}
