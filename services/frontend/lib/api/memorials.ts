import axios from "axios";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:5001";

export const memorialService = {
  async getMemorials(skip = 0, limit = 20) {
    const response = await axios.get(`${API_URL}/api/v1/memorials`, {
      params: { skip, limit },
    });
    return response.data;
  },

  async getMemorialById(id: string) {
    const response = await axios.get(`${API_URL}/api/v1/memorials/${id}`);
    return response.data;
  },

  async getMemorialMedia(memorialId: string) {
    const response = await axios.get(
      `${API_URL}/api/v1/memorials/${memorialId}/media`,
    );
    return response.data;
  },

  async searchMemorials(query: string) {
    const response = await axios.get(`${API_URL}/api/v1/memorials`, {
      params: { search: query },
    });
    return response.data;
  },
};

export default memorialService;
