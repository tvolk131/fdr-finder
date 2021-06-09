import axios from 'axios';
import {ShowFormat, ShowInfo} from './components/showCard';

export const getPodcasts = async (filter: string = '', limit: number = 0, skip: number = 0): Promise<ShowInfo[]> => {
  return (await axios.get(`/api/podcasts?filter=${filter}&limit=${limit}&skip=${skip}`)).data.map((show: any) => ({
    ...show,
    createTime: new Date(show.createTime * 1000),
    showFormat: ShowFormat.Unspecified
  }));
};

export const getPodcastRssUrl = (filter: string) => {
  let url = 'https://fdr-finder.tommyvolk.com/api/podcasts/rss';
  if (filter.length) {
    url += `?filter=${filter}`;
  }
  return encodeURI(url);
}