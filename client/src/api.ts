import axios from 'axios';
import {ShowFormat, ShowInfo} from './components/showCard';

const deserializeShowInfo = (data: any): ShowInfo => {
  return {
    ...data,
    createTime: new Date(data.createTime * 1000),
    showFormat: ShowFormat.Unspecified
  };
};

export const getPodcast = async (podcastNum: number): Promise<ShowInfo> => {
  return deserializeShowInfo((await axios.get(`/api/podcasts/${podcastNum}`)).data);
};

export const getAllPodcasts = async (): Promise<ShowInfo[]> => {
  return (await axios.get('/api/allPodcasts')).data.map(deserializeShowInfo);
}

export const searchPodcasts = async (data: {query?: string, tags?: string[]}): Promise<ShowInfo[]> => {
  const queryParams: {[key: string]: string} = {};
  if (data.query && data.query.length) {
    queryParams['query'] = data.query;
  }
  if (data.tags && data.tags.length) {
    queryParams['tags'] = data.tags.join(',');
  }

  const res = await axios.get(generateUrlWithQueryParams('/api/search/podcasts', queryParams));
  return res.data.map(deserializeShowInfo);
};

export const getPodcastRssUrl = (data: {query?: string, tags?: string[]}) => {
  const queryParams: {[key: string]: string} = {};
  if (data.query && data.query.length) {
    queryParams['query'] = data.query;
  }
  if (data.tags && data.tags.length) {
    queryParams['tags'] = data.tags.join(',');
  }

  return encodeURI(generateUrlWithQueryParams('https://fdr-finder.tommyvolk.com/api/search/podcasts/rss', queryParams));
}

export const getAllTags = async (): Promise<string[]> => {
  return (await axios.get('/api/allTags')).data;
}

export const getFilteredTagsWithCounts = async (tags: string[]): Promise<{tag: string, count: number}[]> => {
  const queryParams: {[key: string]: string} = {};
  if (tags.length) {
    queryParams['tags'] = tags.join(',');
  }
  return (await axios.get(generateUrlWithQueryParams('/api/filteredTagsWithCounts', queryParams))).data;
}

const generateUrlWithQueryParams = (baseUrl: string, queryParams: {[key: string]: string}) => {
  let keys = Object.keys(queryParams);
  if (keys.length) {
    baseUrl += '?';
    baseUrl += keys.map((key) => `${key}=${queryParams[key]}`).join('&');
  }
  return baseUrl;
}