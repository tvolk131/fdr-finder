import axios from 'axios';
import {ShowFormat, ShowInfo} from './components/showCard';

export const getPodcasts = async (filter: string = '', limit: number = 0, skip: number = 0): Promise<ShowInfo[]> => {
  return (await axios.get(`/api/podcasts?filter=${filter}&limit=${limit}&skip=${skip}`)).data.map((show: any) => ({
    ...show,
    createTime: new Date(show.createTime * 1000),
    showFormat: ShowFormat.Unspecified
  }));
};