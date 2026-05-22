import { useMemo, useCallback, useState, useEffect, type ReactNode } from 'react';
import { OrgContext, type OrgContextValue } from '@/shared/hooks/useOrgContext';
import type { Project, CreateProjectRequest, UpdateProjectRequest } from 'shared/remote-types';

export const SEED_ORG_ID = '00000000-0000-0000-0000-000000000001';
export const SEED_PROJECT_ID = '00000000-0000-0000-0000-000000000002';
export const SEED_BACKLOG_STATUS_ID = '00000000-0000-0000-0000-000000000010';
export const SEED_IN_PROGRESS_STATUS_ID = '00000000-0000-0000-0000-000000000011';
export const SEED_REVIEW_STATUS_ID = '00000000-0000-0000-0000-000000000012';
export const SEED_DONE_STATUS_ID = '00000000-0000-0000-0000-000000000013';
export const SEED_TAG_BUG_ID = '00000000-0000-0000-0000-000000000020';
export const SEED_TAG_FEATURE_ID = '00000000-0000-0000-0000-000000000021';
export const SEED_TAG_ENHANCEMENT_ID = '00000000-0000-0000-0000-000000000022';

interface LocalProject {
  id: string;
  organization_id: string;
  name: string;
  color: string;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

interface LocalOrgProviderProps {
  organizationId?: string;
  children: ReactNode;
}

function apiUrl(path: string) {
  return `/api/local${path}`;
}

async function apiGet<T>(path: string): Promise<T> {
  const res = await fetch(apiUrl(path));
  if (!res.ok) throw new Error(`API GET ${path} failed: ${res.status}`);
  const body = await res.json();
  return body.data as T;
}

async function apiPost<T>(path: string, data: unknown): Promise<T> {
  const res = await fetch(apiUrl(path), {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  if (!res.ok) throw new Error(`API POST ${path} failed: ${res.status}`);
  const body = await res.json();
  return body.data as T;
}

function toProject(p: LocalProject): Project {
  return {
    id: p.id,
    organization_id: p.organization_id,
    name: p.name,
    color: p.color,
    sort_order: p.sort_order,
    created_at: p.created_at,
    updated_at: p.updated_at,
  };
}

export function LocalOrgProvider({
  organizationId = SEED_ORG_ID,
  children,
}: LocalOrgProviderProps) {
  const [localProjects, setLocalProjects] = useState<LocalProject[]>([]);

  const loadProjects = useCallback(async () => {
    try {
      const projects = await apiGet<LocalProject[]>(
        `/projects?organization_id=${organizationId}`
      );
      setLocalProjects(projects);
    } catch { /* server may not be ready */ }
  }, [organizationId]);

  useEffect(() => {
    loadProjects();
  }, [loadProjects]);

  const projects = useMemo(() => localProjects.map(toProject), [localProjects]);

  const projectsById = useMemo(() => {
    const map = new Map<string, Project>();
    for (const p of projects) map.set(p.id, p);
    return map;
  }, [projects]);

  const getProject = useCallback(
    (projectId: string) => projectsById.get(projectId),
    [projectsById]
  );

  const insertProject = useCallback(
    (data: CreateProjectRequest) => {
      const promise = apiPost<LocalProject>('/projects', data).then((created) => {
        setLocalProjects((prev) => [...prev, created]);
        return created;
      });
      return { data: toProject({ id: '', organization_id: data.organization_id, name: data.name, color: data.color ?? '#6b7280', sort_order: 0, created_at: '', updated_at: '' }), persisted: promise.then(toProject) };
    },
    []
  );

  const updateProject = useCallback(
    (id: string, changes: Partial<UpdateProjectRequest>) => {
      const promise = fetch(apiUrl(`/projects/${id}`), {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(changes),
      }).then(() => loadProjects());
      return { persisted: promise };
    },
    [loadProjects]
  );

  const removeProject = useCallback(
    (id: string) => {
      const promise = fetch(apiUrl(`/projects/${id}`), { method: 'DELETE' }).then(() => {
        setLocalProjects((prev) => prev.filter((p) => p.id !== id));
      });
      return { persisted: promise };
    },
    []
  );

  const value = useMemo<OrgContextValue>(
    () => ({
      organizationId,
      projects,
      isLoading: false,
      error: null,
      retry: loadProjects,
      insertProject: insertProject as OrgContextValue['insertProject'],
      updateProject: updateProject as OrgContextValue['updateProject'],
      removeProject: removeProject as OrgContextValue['removeProject'],
      getProject,
      projectsById,
      membersWithProfilesById: new Map(),
    }),
    [organizationId, projects, insertProject, updateProject, removeProject, getProject, projectsById, loadProjects]
  );

  return <OrgContext.Provider value={value}>{children}</OrgContext.Provider>;
}