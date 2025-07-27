export interface Tuna {
  id: string;
  user: {
    name: string;
    handle: string;
    avatar: string;
  };
  content: string;
  timestamp: string;
  image?: string;
  likes: number;
  retunas: number;
  comments: number;
}
