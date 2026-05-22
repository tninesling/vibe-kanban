import { useMemo, useCallback, useState, useEffect, type ReactNode } from 'react';
import { ProjectContext, type ProjectContextValue } from '@/shared/hooks/useProjectContext';
import type {
  Issue,
  ProjectStatus,
  Tag,
  IssueAssignee,
  IssueFollower,
  IssueTag,
  IssueRelationship,
  PullRequest,
  PullRequestIssue,
  Workspace,
  CreateIssueRequest,
  UpdateIssueRequest,
} from 'shared/remote-types';
import {
  SEED_PROJECT_ID,
  SEED_BACKLOG_STATUS_ID,
  SEED_IN_PROGRESS_STATUS_ID,
  SEED_REVIEW_STATUS_ID,
  SEED_DONE_STATUS_ID,
  SEED_TAG_BUG_ID,
  SEED_TAG_FEATURE_ID,
  SEED_TAG_ENHANCEMENT_ID,
} from './LocalOrgProvider';

const SEED_STATUSES: ProjectStatus[] = [
  { id: SEED_BACKLOG_STATUS_ID, project_id: SEED_PROJECT_ID, name: 'Backlog', color: '#6b7280', sort_order: 0, hidden: false, created_at: '' },
  { id: SEED_IN_PROGRESS_STATUS_ID, project_id: SEED_PROJECT_ID, name: 'In Progress', color: '#3b82f6', sort_order: 1, hidden: false, created_at: '' },
  { id: SEED_REVIEW_STATUS_ID, project_id: SEED_PROJECT_ID, name: 'Review', color: '#f59e0b', sort_order: 2, hidden: false, created_at: '' },
  { id: SEED_DONE_STATUS_ID, project_id: SEED_PROJECT_ID, name: 'Done', color: '#22c55e', sort_order: 3, hidden: false, created_at: '' },
];

const SEED_TAGS: Tag[] = [
  { id: SEED_TAG_BUG_ID, project_id: SEED_PROJECT_ID, name: 'Bug', color: '#ef4444' },
  { id: SEED_TAG_FEATURE_ID, project_id: SEED_PROJECT_ID, name: 'Feature', color: '#22c55e' },
  { id: SEED_TAG_ENHANCEMENT_ID, project_id: SEED_PROJECT_ID, name: 'Enhancement', color: '#8b5cf6' },
];

interface LocalIssueRow {
  id: string;
  project_id: string;
  issue_number: number;
  simple_id: string;
  status_id: string;
  title: string;
  description: string | null;
  priority: string | null;
  sort_order: number;
  parent_issue_id: string | null;
  parent_issue_sort_order: number | null;
  start_date: string | null;
  target_date: string | null;
  completed_at: string | null;
  extension_metadata: string | null;
  creator_user_id: string | null;
  created_at: string;
  updated_at: string;
}

interface LocalStatusRow {
  id: string;
  project_id: string;
  name: string;
  color: string;
  sort_order: number;
  hidden: boolean;
  created_at: string;
}

interface LocalTagRow {
  id: string;
  project_id: string;
  name: string;
  color: string;
}

interface LocalProjectProviderProps {
  projectId?: string;
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

function toIssue(row: LocalIssueRow): Issue {
  return row as unknown as Issue;
}

function toStatus(row: LocalStatusRow): ProjectStatus {
  return {
    id: row.id,
    project_id: row.project_id,
    name: row.name,
    color: row.color,
    sort_order: row.sort_order,
    hidden: row.hidden,
    created_at: row.created_at,
  };
}

function toTag(row: LocalTagRow): Tag {
  return { id: row.id, project_id: row.project_id, name: row.name, color: row.color };
}

export function LocalProjectProvider({
  projectId = SEED_PROJECT_ID,
  children,
}: LocalProjectProviderProps) {
  const [issues, setIssues] = useState<Issue[]>([]);
  const [statuses, setStatuses] = useState<ProjectStatus[]>(SEED_STATUSES);
  const [tags, setTags] = useState<Tag[]>(SEED_TAGS);
  const [issueTagsList, setIssueTagsList] = useState<IssueTag[]>([]);

  const loadIssues = useCallback(async () => {
    try {
      const rows = await apiGet<LocalIssueRow[]>(`/projects/${projectId}/issues`);
      setIssues(rows.map(toIssue));
    } catch { /* not ready */ }
  }, [projectId]);

  const loadStatuses = useCallback(async () => {
    try {
      const rows = await apiGet<LocalStatusRow[]>(`/projects/${projectId}/statuses`);
      setStatuses(rows.map(toStatus));
    } catch { /* not ready */ }
  }, [projectId]);

  const loadTags = useCallback(async () => {
    try {
      const rows = await apiGet<LocalTagRow[]>(`/projects/${projectId}/tags`);
      setTags(rows.map(toTag));
    } catch { /* not ready */ }
  }, [projectId]);

  const loadIssueTags = useCallback(async () => {
    try {
      const rows = await apiGet<IssueTag[]>(`/issue-tags/by-project/${projectId}`);
      setIssueTagsList(rows);
    } catch { /* not ready */ }
  }, [projectId]);

  useEffect(() => {
    loadIssues();
    loadStatuses();
    loadTags();
    loadIssueTags();
  }, [loadIssues, loadStatuses, loadTags, loadIssueTags]);

  const issuesById = useMemo(() => {
    const map = new Map<string, Issue>();
    for (const issue of issues) map.set(issue.id, issue);
    return map;
  }, [issues]);

  const statusesById = useMemo(() => {
    const map = new Map<string, ProjectStatus>();
    for (const s of statuses) map.set(s.id, s);
    return map;
  }, [statuses]);

  const tagsById = useMemo(() => {
    const map = new Map<string, Tag>();
    for (const t of tags) map.set(t.id, t);
    return map;
  }, [tags]);

  const getIssue = useCallback(
    (issueId: string) => issuesById.get(issueId),
    [issuesById]
  );

  const getIssuesForStatus = useCallback(
    (statusId: string) => issues.filter((i) => i.status_id === statusId),
    [issues]
  );

  const getAssigneesForIssue = useCallback(() => [] as IssueAssignee[], []);
  const getFollowersForIssue = useCallback(() => [] as IssueFollower[], []);

  const getTagsForIssue = useCallback(
    (issueId: string) => issueTagsList.filter((t) => t.issue_id === issueId),
    [issueTagsList]
  );

  const getTagObjectsForIssue = useCallback(
    (issueId: string) => {
      const its = issueTagsList.filter((t) => t.issue_id === issueId);
      return its.map((it) => tagsById.get(it.tag_id)).filter((t): t is Tag => t !== undefined);
    },
    [issueTagsList, tagsById]
  );

  const getRelationshipsForIssue = useCallback(() => [] as IssueRelationship[], []);
  const getStatus = useCallback((statusId: string) => statusesById.get(statusId), [statusesById]);
  const getTag = useCallback((tagId: string) => tagsById.get(tagId), [tagsById]);
  const getPullRequestsForIssue = useCallback(() => [] as PullRequest[], []);
  const getWorkspacesForIssue = useCallback(() => [] as Workspace[], []);

  const insertIssue = useCallback(
    (data: CreateIssueRequest) => {
      const promise = fetch(apiUrl(`/projects/${projectId}/issues`), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
      })
        .then((r) => r.json())
        .then((body) => {
          const created = body.data as LocalIssueRow;
          const issue = toIssue(created);
          setIssues((prev) => [...prev, issue]);
          return issue;
        });
      return {
        data: { id: '', project_id: projectId, issue_number: 0, simple_id: '', status_id: data.status_id ?? SEED_BACKLOG_STATUS_ID, title: data.title ?? '', description: data.description ?? null, priority: data.priority ?? null, sort_order: data.sort_order ?? 0, parent_issue_id: data.parent_issue_id ?? null, parent_issue_sort_order: data.parent_issue_sort_order ?? null, start_date: null, target_date: null, completed_at: null, extension_metadata: null, creator_user_id: null, created_at: '', updated_at: '' } as Issue,
        persisted: promise,
      };
    },
    [projectId]
  );

  const updateIssue = useCallback(
    (id: string, changes: Partial<UpdateIssueRequest>) => {
      const promise = fetch(apiUrl(`/issues/${id}`), {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(changes),
      })
        .then((r) => r.json())
        .then((body) => {
          const updated = body.data as LocalIssueRow;
          const issue = toIssue(updated);
          setIssues((prev) => prev.map((i) => (i.id === id ? issue : i)));
          return;
        });
      return { persisted: promise };
    },
    []
  );

  const removeIssue = useCallback(
    (id: string) => {
      const promise = fetch(apiUrl(`/issues/${id}`), { method: 'DELETE' }).then(() => {
        setIssues((prev) => prev.filter((i) => i.id !== id));
      });
      return { persisted: promise };
    },
    []
  );

  const insertStatus = useCallback(
    (data: { project_id: string; name: string; color: string; sort_order?: number }) => {
      const promise = fetch(apiUrl('/statuses'), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
      })
        .then((r) => r.json())
        .then((body) => {
          const created = body.data as LocalStatusRow;
          setStatuses((prev) => [...prev, toStatus(created)]);
          return toStatus(created);
        });
      return { data: { id: '', project_id: data.project_id, name: data.name, color: data.color, sort_order: data.sort_order ?? 0, hidden: false, created_at: '' } as ProjectStatus, persisted: promise };
    },
    []
  );

  const updateStatus = useCallback(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (id: string, changes: Record<string, any>) => {
      const promise = fetch(apiUrl(`/statuses/${id}`), {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(changes),
      })
        .then((r) => r.json())
        .then((body) => {
          const updated = body.data as LocalStatusRow;
          setStatuses((prev) => prev.map((s) => (s.id === id ? toStatus(updated) : s)));
          return;
        });
      return { persisted: promise };
    },
    []
  );

  const removeStatus = useCallback(
    (id: string) => {
      const promise = fetch(apiUrl(`/statuses/${id}`), { method: 'DELETE' }).then(() => {
        setStatuses((prev) => prev.filter((s) => s.id !== id));
      });
      return { persisted: promise };
    },
    []
  );
  const insertTag = useCallback(
    (data: { project_id: string; name: string; color: string }) => {
      const promise = fetch(apiUrl(`/projects/${projectId}/tags`), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
      })
        .then((r) => r.json())
        .then((body) => {
          const created = body.data as LocalTagRow;
          setTags((prev) => [...prev, toTag(created)]);
          return toTag(created);
        });
      return { data: { id: '', project_id: projectId, name: data.name, color: data.color } as Tag, persisted: promise };
    },
    [projectId]
  );
  const updateTag = useCallback(() => ({ persisted: Promise.resolve() }), []);
  const removeTag = useCallback(() => ({ persisted: Promise.resolve() }), []);
  const insertIssueAssignee = useCallback(() => ({ data: {} as IssueAssignee, persisted: Promise.resolve({} as IssueAssignee) }), []);
  const removeIssueAssignee = useCallback(() => ({ persisted: Promise.resolve() }), []);
  const insertIssueFollower = useCallback(() => ({ data: {} as IssueFollower, persisted: Promise.resolve({} as IssueFollower) }), []);
  const removeIssueFollower = useCallback(() => ({ persisted: Promise.resolve() }), []);
  const insertIssueTag = useCallback(
    (data: { issue_id: string; tag_id: string }) => {
      const promise = fetch(apiUrl('/issue-tags'), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
      })
        .then((r) => r.json())
        .then((body) => {
          const created = body.data as IssueTag;
          setIssueTagsList((prev) => [...prev, created]);
          return created;
        });
      return { data: { id: '', issue_id: data.issue_id, tag_id: data.tag_id } as IssueTag, persisted: promise };
    },
    []
  );

  const removeIssueTag = useCallback(
    (id: string) => {
      const promise = fetch(apiUrl(`/issue-tags/${id}`), { method: 'DELETE' }).then(() => {
        setIssueTagsList((prev) => prev.filter((t) => t.id !== id));
      });
      return { persisted: promise };
    },
    []
  );
  const insertIssueRelationship = useCallback(() => ({ data: {} as IssueRelationship, persisted: Promise.resolve({} as IssueRelationship) }), []);
  const removeIssueRelationship = useCallback(() => ({ persisted: Promise.resolve() }), []);
  const insertPullRequestIssue = useCallback(() => ({ data: {} as PullRequestIssue, persisted: Promise.resolve({} as PullRequestIssue) }), []);
  const removePullRequestIssue = useCallback(() => ({ persisted: Promise.resolve() }), []);

  const value = useMemo<ProjectContextValue>(
    () => ({
      projectId,
      issues,
      statuses,
      tags,
      issueAssignees: [],
      issueFollowers: [],
      issueTags: issueTagsList,
      issueRelationships: [],
      pullRequests: [],
      pullRequestIssues: [],
      workspaces: [],
      isLoading: false,
      error: null,
      retry: () => { loadIssues(); loadStatuses(); loadTags(); loadIssueTags(); },
      insertIssue,
      updateIssue,
      removeIssue,
      insertStatus,
      updateStatus,
      removeStatus,
      insertTag,
      updateTag,
      removeTag,
      insertIssueAssignee,
      removeIssueAssignee,
      insertIssueFollower,
      removeIssueFollower,
      insertIssueTag,
      removeIssueTag,
      insertIssueRelationship,
      removeIssueRelationship,
      insertPullRequestIssue,
      removePullRequestIssue,
      getIssue,
      getIssuesForStatus,
      getAssigneesForIssue,
      getFollowersForIssue,
      getTagsForIssue,
      getTagObjectsForIssue,
      getRelationshipsForIssue,
      getStatus,
      getTag,
      getPullRequestsForIssue,
      getWorkspacesForIssue,
      issuesById,
      statusesById,
      tagsById,
    }),
    [
      projectId, issues, statuses, tags, issueTagsList,
      insertIssue, updateIssue, removeIssue,
      insertStatus, updateStatus, removeStatus,
      insertTag, updateTag, removeTag,
      insertIssueAssignee, removeIssueAssignee,
      insertIssueFollower, removeIssueFollower,
      insertIssueTag, removeIssueTag,
      insertIssueRelationship, removeIssueRelationship,
      insertPullRequestIssue, removePullRequestIssue,
      getIssue, getIssuesForStatus, getAssigneesForIssue,
      getFollowersForIssue, getTagsForIssue, getTagObjectsForIssue,
      getRelationshipsForIssue, getStatus, getTag,
      getPullRequestsForIssue, getWorkspacesForIssue,
      issuesById, statusesById, tagsById,
loadIssues, loadStatuses, loadTags, loadIssueTags,
    ]
  );

  return (
    <ProjectContext.Provider value={value}>{children}</ProjectContext.Provider>
  );
}