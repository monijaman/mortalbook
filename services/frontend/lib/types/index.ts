export interface Memorial {
  id: string;
  name: string;
  date_of_birth: string;
  date_of_death: string;
  biography: string;
  created_by: string;
  created_at: string;
  updated_at: string;
  status: string;
}

export interface MemorialMedia {
  id: string;
  memorial_id: string;
  url: string;
  media_type: "photo" | "video" | "document";
  description: string;
  created_at: string;
}
