import { useState } from "react";
import { Project, Session } from "../lib/tauri";
import { Icon } from "./Icon";

export type Selection =
  | { kind: "all" }
  | { kind: "none" }
  | { kind: "project"; project: string }
  | { kind: "front"; project: string; front: string };

interface Props {
  projects: Project[];
  sessions: Session[];
  selection: Selection;
  onSelect: (s: Selection) => void;
  onAddProject: () => void;
}

export function matches(s: Session, sel: Selection) {
  switch (sel.kind) {
    case "all":
      return true;
    case "none":
      return s.project === null;
    case "project":
      return s.project === sel.project;
    case "front":
      return s.project === sel.project && s.front === sel.front;
  }
}

export function Sidebar({ projects, sessions, selection, onSelect, onAddProject }: Props) {
  // Projetos abertos na arvore; o selecionado abre sozinho.
  const [expanded, setExpanded] = useState<Record<string, boolean>>({});
  const count = (sel: Selection) => sessions.filter((s) => matches(s, sel)).length;
  const isSelected = (sel: Selection) => JSON.stringify(sel) === JSON.stringify(selection);
  const legacy = count({ kind: "none" });

  return (
    <nav className="sidebar" aria-label="Projetos">
      <div className="sidebar-brand">ReCapt</div>

      <Item label="Todas as sessões" count={sessions.length} selected={isSelected({ kind: "all" })} onClick={() => onSelect({ kind: "all" })} />

      <div className="sidebar-section">
        <span>Projetos</span>
      </div>

      {projects.length === 0 && <p className="sidebar-empty">Crie um projeto para organizar suas reuniões por frente e data.</p>}

      {projects.map((p) => {
        const sel: Selection = { kind: "project", project: p.name };
        const open = expanded[p.name] ?? (selection.kind !== "all" && "project" in selection && selection.project === p.name);
        return (
          <div key={p.name}>
            <Item
              label={p.name}
              count={count(sel)}
              selected={isSelected(sel)}
              onClick={() => {
                onSelect(sel);
                setExpanded((e) => ({ ...e, [p.name]: true }));
              }}
              toggle={p.fronts.length > 0 ? { open, onToggle: () => setExpanded((e) => ({ ...e, [p.name]: !open })) } : undefined}
            />
            {open &&
              p.fronts.map((f) => {
                const fsel: Selection = { kind: "front", project: p.name, front: f };
                return (
                  <Item key={f} label={f} count={count(fsel)} selected={isSelected(fsel)} indent onClick={() => onSelect(fsel)} />
                );
              })}
          </div>
        );
      })}

      <button className="sidebar-add" onClick={onAddProject}>
        <Icon name="plus" size={16} />
        Adicionar projeto
      </button>

      {legacy > 0 && (
        <>
          <div className="sidebar-section">
            <span>Outras</span>
          </div>
          <Item label="Sem projeto" count={legacy} selected={isSelected({ kind: "none" })} onClick={() => onSelect({ kind: "none" })} />
        </>
      )}
    </nav>
  );
}

interface ItemProps {
  label: string;
  count: number;
  selected: boolean;
  indent?: boolean;
  onClick: () => void;
  toggle?: { open: boolean; onToggle: () => void };
}

function Item({ label, count, selected, indent, onClick, toggle }: ItemProps) {
  return (
    <div className={`sidebar-item ${selected ? "is-selected" : ""} ${indent ? "is-indented" : ""}`}>
      {toggle ? (
        <button className="sidebar-toggle" onClick={toggle.onToggle} aria-label={toggle.open ? "Recolher" : "Expandir"} aria-expanded={toggle.open}>
          <Icon name={toggle.open ? "chevronDown" : "chevronRight"} size={14} />
        </button>
      ) : (
        <span className="sidebar-toggle" />
      )}
      <button className="sidebar-label" onClick={onClick} aria-current={selected ? "true" : undefined} title={label}>
        <span>{label}</span>
        <span className="sidebar-count">{count}</span>
      </button>
    </div>
  );
}
