import { useState } from "react";
import { Project, Session } from "../lib/tauri";
import { Icon } from "./Icon";

export type Selection =
  | { kind: "all" }
  | { kind: "none" }
  | { kind: "project"; project: string }
  | { kind: "front"; project: string; front: string }
  /** `theme: null` sao as sessoes gravadas direto na frente, antes do nivel Tema. */
  | { kind: "theme"; project: string; front: string; theme: string | null };

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
    case "theme":
      return s.project === sel.project && s.front === sel.front && s.theme === sel.theme;
  }
}

export function Sidebar({ projects, sessions, selection, onSelect, onAddProject }: Props) {
  // Projetos abertos na arvore; o selecionado abre sozinho.
  const [expanded, setExpanded] = useState<Record<string, boolean>>({});
  const toggle = (key: string, open: boolean) => setExpanded((e) => ({ ...e, [key]: !open }));
  const inSelection = (project: string, front?: string) =>
    "project" in selection && selection.project === project && (front === undefined || ("front" in selection && selection.front === front));
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
        const open = expanded[p.name] ?? inSelection(p.name);
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
              toggle={p.fronts.length > 0 ? { open, onToggle: () => toggle(p.name, open) } : undefined}
            />
            {open &&
              p.fronts.map((f) => {
                const key = `${p.name}/${f.name}`;
                const fsel: Selection = { kind: "front", project: p.name, front: f.name };
                const legacySel: Selection = { kind: "theme", project: p.name, front: f.name, theme: null };
                const legacyCount = count(legacySel);
                const fopen = expanded[key] ?? inSelection(p.name, f.name);
                const hasChildren = f.themes.length > 0 || legacyCount > 0;
                return (
                  <div key={f.name}>
                    <Item
                      label={f.name}
                      count={count(fsel)}
                      selected={isSelected(fsel)}
                      depth={1}
                      onClick={() => {
                        onSelect(fsel);
                        setExpanded((e) => ({ ...e, [key]: true }));
                      }}
                      toggle={hasChildren ? { open: fopen, onToggle: () => toggle(key, fopen) } : undefined}
                    />
                    {fopen &&
                      f.themes.map((t) => {
                        const tsel: Selection = { kind: "theme", project: p.name, front: f.name, theme: t };
                        return <Item key={t} label={t} count={count(tsel)} selected={isSelected(tsel)} depth={2} onClick={() => onSelect(tsel)} />;
                      })}
                    {fopen && legacyCount > 0 && (
                      <Item label="Sem tema" count={legacyCount} selected={isSelected(legacySel)} depth={2} muted onClick={() => onSelect(legacySel)} />
                    )}
                  </div>
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
  /** 0 projeto, 1 frente, 2 tema. */
  depth?: number;
  muted?: boolean;
  onClick: () => void;
  toggle?: { open: boolean; onToggle: () => void };
}

function Item({ label, count, selected, depth = 0, muted, onClick, toggle }: ItemProps) {
  return (
    <div
      className={`sidebar-item ${selected ? "is-selected" : ""} ${muted ? "is-muted" : ""}`}
      style={{ paddingLeft: depth * 14 }}
    >
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
