import { Notice } from "../hooks/useNotices";
import { Icon } from "./Icon";

export function Notices({ notices, onDismiss }: { notices: Notice[]; onDismiss: (id: number) => void }) {
  return (
    <div className="notices" aria-live="polite">
      {notices.map((n) => (
        <div key={n.id} className={`notice is-${n.kind}`} role={n.kind === "error" ? "alert" : "status"}>
          <span>{n.text}</span>
          <button className="icon-button" onClick={() => onDismiss(n.id)} aria-label="Fechar aviso">
            <Icon name="close" size={14} />
          </button>
        </div>
      ))}
    </div>
  );
}
