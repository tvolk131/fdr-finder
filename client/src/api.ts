import axios from 'axios';
import {ShowFormat, ShowInfo} from './components/showCard';
import {queryFieldName, tagsFieldName} from './constants';

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

export const getRecentPodcasts = async (amount?: number): Promise<ShowInfo[]> => {
  return (await axios.get(generateUrlWithQueryParams('/api/recentPodcasts', {amount}))).data.map(deserializeShowInfo);
}

export const searchPodcasts = async (data: {query?: string, tags?: string[]}): Promise<ShowInfo[]> => {
  const queryParams: {[key: string]: string} = {};
  if (data.query && data.query.length) {
    queryParams[queryFieldName] = data.query;
  }
  if (data.tags && data.tags.length) {
    queryParams[tagsFieldName] = data.tags.join(',');
  }

  const res = await axios.get(generateUrlWithQueryParams('/api/search/podcasts', queryParams));
  return res.data.map(deserializeShowInfo);
};

export const searchPodcastsAutocomplete = async (query: string): Promise<string[]> => {
  const res = await axios.get(generateUrlWithQueryParams('/api/search/podcasts/autocomplete', {query}));
  return res.data;
};

export const getPodcastRssUrl = (data: {query?: string, tags?: string[]}) => {
  const queryParams: {[key: string]: string} = {};
  if (data.query && data.query.length) {
    queryParams[queryFieldName] = data.query;
  }
  if (data.tags && data.tags.length) {
    queryParams[tagsFieldName] = data.tags.join(',');
  }

  return encodeURI(generateUrlWithQueryParams('https://fdr-finder.tommyvolk.com/api/search/podcasts/rss', queryParams));
}

export const getFilteredTagsWithCounts =
async (data: {query?: string, tags?: string[]}): Promise<{tag: string, count: number}[]> => {
  const queryParams: {[key: string]: string} = {};
  if (data.query && data.query.length) {
    queryParams[queryFieldName] = data.query;
  }
  if (data.tags && data.tags.length) {
    queryParams[tagsFieldName] = data.tags.join(',');
  }

  return (await axios.get(generateUrlWithQueryParams('/api/filteredTagsWithCounts', queryParams))).data;
}

export const generateUrlWithQueryParams = (baseUrl: string, queryParams: {[key: string]: string | number | undefined}) => {
  let keys = Object.keys(queryParams);
  keys.forEach((key) => {
    const value = queryParams[key];
    if (value === undefined || (typeof value === 'string' && value.length === 0)) {
      delete queryParams[key];
    }
  });
  keys = Object.keys(queryParams);
  if (keys.length) {
    baseUrl += '?';
    baseUrl += keys.map((key) => `${key}=${queryParams[key]}`).join('&');
  }
  return baseUrl;
}