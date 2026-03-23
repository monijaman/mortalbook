import axios, { AxiosProgressEvent } from "axios";
import { Memorial } from "../store/modules/memorials";

const API_URL = import.meta.env.VITE_API_URL || "http://localhost:5002";

const apiClient = axios.create({ baseURL: API_URL });

apiClient.interceptors.request.use((config) => {
  const token = localStorage.getItem("token");
  if (token) config.headers.Authorization = `Bearer ${token}`;
  return config;
});

apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem("token");
      localStorage.removeItem("refreshToken");
      window.location.href = "/login";
    }
    return Promise.reject(error);
  },
);

// ── Types ──────────────────────────────────────────────────────────────────────

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  skip: number;
  limit: number;
}

export interface User {
  id: string;
  email: string;
  username: string;
  is_admin: boolean;
}

export interface AnalyticsDashboard {
  total_events: number;
  period_days: number;
}

export interface AnalyticsEvent {
  id: string;
  event_type: string;
  created_at: string;
}

export interface PresignedUrlResponse {
  upload_url: string;
  public_url: string;
}

// ── Memorial Service ───────────────────────────────────────────────────────────

export const memorialService = {
  async getMemorials(
    skip = 0,
    limit = 20,
    search = "",
  ): Promise<PaginatedResponse<Memorial>> {
    const params: Record<string, unknown> = { skip, limit };
    if (search.trim()) params.search = search.trim();
    const res = await apiClient.get("/api/v1/memorials/", { params });
    return res.data;
  },

  async getMemorial(id: string): Promise<Memorial> {
    const res = await apiClient.get(`/api/v1/memorials/${id}`);
    return res.data;
  },

  async createMemorial(payload: Partial<Memorial>): Promise<Memorial> {
    const res = await apiClient.post("/api/v1/memorials/", payload);
    return res.data;
  },

  async updateMemorial(
    id: string,
    payload: Partial<Memorial>,
  ): Promise<Memorial> {
    const res = await apiClient.patch(`/api/v1/memorials/${id}`, payload);
    return res.data;
  },

  async updateMemorialStatus(id: string, status: string): Promise<Memorial> {
    const res = await apiClient.put(`/api/v1/memorials/${id}/status`, {
      status,
    });
    return res.data;
  },

  async deleteMemorial(id: string): Promise<void> {
    await apiClient.delete(`/api/v1/memorials/${id}`);
  },

  async getPresignedUploadUrl(
    memorialId: string,
    fileName: string,
    contentType: string,
  ): Promise<PresignedUrlResponse> {
    const res = await apiClient.post(
      `/api/v1/memorials/${memorialId}/media/presigned-url`,
      { file_name: fileName, content_type: contentType },
    );
    return res.data;
  },

  async uploadFileToS3(
    presignedUrl: string,
    file: File,
    onProgress?: (pct: number) => void,
  ): Promise<void> {
    await axios.put(presignedUrl, file, {
      headers: { "Content-Type": file.type },
      onUploadProgress: (evt: AxiosProgressEvent) => {
        if (onProgress && evt.total) {
          onProgress(Math.round((evt.loaded * 100) / evt.total));
        }
      },
    });
  },
};

// ── User Service ───────────────────────────────────────────────────────────────

export const userService = {
  async getUsers(skip = 0, limit = 20): Promise<PaginatedResponse<User>> {
    const res = await apiClient.get("/api/v1/admin/users", {
      params: { skip, limit },
    });
    return res.data;
  },

  async deleteUser(id: string): Promise<void> {
    await apiClient.delete(`/api/v1/admin/users/${id}`);
  },
};

// ── Analytics Service ──────────────────────────────────────────────────────────

export const analyticsService = {
  async getDashboard(days = 30): Promise<AnalyticsDashboard> {
    const res = await apiClient.get("/api/v1/analytics/dashboard", {
      params: { days },
    });
    return res.data;
  },

  async getEvents(
    skip = 0,
    limit = 50,
  ): Promise<PaginatedResponse<AnalyticsEvent>> {
    const res = await apiClient.get("/api/v1/analytics/events", {
      params: { skip, limit },
    });
    return res.data;
  },
};
