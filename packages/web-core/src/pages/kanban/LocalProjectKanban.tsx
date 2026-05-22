import { Group, Layout, Panel, Separator } from 'react-resizable-panels';
import { useCurrentKanbanRouteState } from '@/shared/hooks/useCurrentKanbanRouteState';
import { useOrgContext } from '@/shared/hooks/useOrgContext';
import { usePageTitle } from '@/shared/hooks/usePageTitle';
import { useIsMobile } from '@/shared/hooks/useIsMobile';
import { useProjectContext } from '@/shared/hooks/useProjectContext';
import { KanbanContainer } from '@/features/kanban/ui/KanbanContainer';
import { ProjectRightSidebarContainer } from '@/pages/kanban/ProjectRightSidebarContainer';
import {
  PERSIST_KEYS,
  usePaneSize,
} from '@/shared/stores/useUiPreferencesStore';
import {
  LocalOrgProvider,
  SEED_PROJECT_ID,
} from '@/shared/providers/local/LocalOrgProvider';
import { LocalProjectProvider } from '@/shared/providers/local/LocalProjectProvider';

function LocalKanbanBoard() {
  return (
    <div className="flex h-full min-h-0 w-full flex-col">
      <div className="min-h-0 flex-1">
        <KanbanContainer />
      </div>
    </div>
  );
}

function LocalKanbanLayout({ projectName }: { projectName: string }) {
  const { issueId, isPanelOpen } = useCurrentKanbanRouteState();
  const isMobile = useIsMobile();
  const { getIssue } = useProjectContext();
  const issue = issueId ? getIssue(issueId) : undefined;
  usePageTitle(issue?.title, projectName);
  const [kanbanLeftPanelSize, setKanbanLeftPanelSize] = usePaneSize(
    PERSIST_KEYS.kanbanLeftPanel,
    75
  );

  const isRightPanelOpen = isPanelOpen;

  if (isMobile) {
    return isRightPanelOpen ? (
      <div className="h-full w-full overflow-hidden bg-secondary">
        <ProjectRightSidebarContainer />
      </div>
    ) : (
      <div className="h-full w-full overflow-hidden bg-primary">
        <LocalKanbanBoard />
      </div>
    );
  }

  const kanbanDefaultLayout: Layout =
    typeof kanbanLeftPanelSize === 'number'
      ? {
          'kanban-left': kanbanLeftPanelSize,
          'kanban-right': 100 - kanbanLeftPanelSize,
        }
      : { 'kanban-left': 75, 'kanban-right': 25 };

  const onKanbanLayoutChange = (layout: Layout) => {
    if (isRightPanelOpen) {
      setKanbanLeftPanelSize(layout['kanban-left']);
    }
  };

  return (
    <Group
      orientation="horizontal"
      className="flex-1 min-w-0 h-full"
      defaultLayout={kanbanDefaultLayout}
      onLayoutChange={onKanbanLayoutChange}
    >
      <Panel
        id="kanban-left"
        minSize="20%"
        className="min-w-0 h-full overflow-hidden bg-primary"
      >
        <LocalKanbanBoard />
      </Panel>

      {isRightPanelOpen && (
        <Separator
          id="kanban-separator"
          className="w-1 bg-panel outline-none hover:bg-brand/50 transition-colors cursor-col-resize"
        />
      )}

      {isRightPanelOpen && (
        <Panel
          id="kanban-right"
          minSize="400px"
          maxSize="800px"
          className="min-w-0 h-full overflow-hidden bg-secondary"
        >
          <ProjectRightSidebarContainer />
        </Panel>
      )}
    </Group>
  );
}

function LocalProjectNameLoader({
  projectId,
  children,
}: {
  projectId: string;
  children: (name: string) => React.ReactNode;
}) {
  const { projects } = useOrgContext();
  const projectName = projects.find((p) => p.id === projectId)?.name ?? 'Project';
  return <>{children(projectName)}</>;
}

export function LocalProjectKanban() {
  const { projectId } = useCurrentKanbanRouteState();
  const resolvedProjectId = projectId ?? SEED_PROJECT_ID;

  return (
    <LocalOrgProvider>
      <LocalProjectProvider projectId={resolvedProjectId}>
        <LocalProjectNameLoader projectId={resolvedProjectId}>
          {(projectName) => <LocalKanbanLayout projectName={projectName} />}
        </LocalProjectNameLoader>
      </LocalProjectProvider>
    </LocalOrgProvider>
  );
}