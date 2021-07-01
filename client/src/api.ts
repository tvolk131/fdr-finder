import axios from 'axios';
import {ShowFormat, ShowInfo} from './components/showCard';

const deserializeShowInfo = (data: any): ShowInfo => {
  return {
    ...data,
    createTime: new Date(data.createTime * 1000),
    showFormat: ShowFormat.Unspecified
  };
};

export const searchPodcasts = async (query: string = '', limit: number = 0, skip: number = 0): Promise<ShowInfo[]> => {
  const res = await axios.get(`/api/search/podcasts?query=${query}&limit=${limit}&skip=${skip}`);
  return res.data.map(deserializeShowInfo);
};

export const getPodcast = async (podcastNum: number): Promise<ShowInfo> => {
  return deserializeShowInfo((await axios.get(`/api/podcasts/${podcastNum}`)).data);
};

export const getAllPodcasts = async (): Promise<ShowInfo[]> => {
  return (await axios.get('/api/allPodcasts')).data.map(deserializeShowInfo);
}

export const getPodcastRssUrl = (query: string) => {
  let url = 'https://fdr-finder.tommyvolk.com/api/search/podcasts/rss';
  if (query.length) {
    url += `?query=${query}`;
  }
  return encodeURI(url);
}