import axios from 'axios';
import {ShowInfo} from './components/showCard';
import {
  queryFieldName,
  limitFieldName,
  offsetFieldName,
  tagsFieldName,
  minLengthSecondsFieldName,
  maxLengthSecondsFieldName,
  filterFieldName
} from './constants';

const deserializeShowInfo = (data: any): ShowInfo => {
  return {
    ...data,
    createTime: new Date(data.createTime * 1000)
  };
};

export const getPodcast = async (podcastNum: number): Promise<ShowInfo> => {
  return deserializeShowInfo((await axios.get(`/api/podcasts/${podcastNum}`)).data);
};

interface SearchResult {
  hits: ShowInfo[],
  totalHits: number,
  totalHitsIsApproximate: boolean,
  processingTimeMs: number
}

export const searchPodcasts =
async (data: {
  query?: string,
  limit?: number,
  offset?: number,
  tags?: string[],
  minLengthSeconds: number | undefined,
  maxLengthSeconds: number | undefined
}): Promise<SearchResult> => {
  const queryParams: {[key: string]: string | number} = {};
  if (data.query && data.query.length) {
    queryParams[queryFieldName] = data.query;
  }
  if (data.limit !== undefined) {
    queryParams[limitFieldName] = data.limit;
  }
  if (data.offset !== undefined) {
    queryParams[offsetFieldName] = data.offset;
  }
  if (data.tags && data.tags.length) {
    queryParams[tagsFieldName] = data.tags.join(',');
  }
  if (data.minLengthSeconds !== undefined) {
    queryParams[minLengthSecondsFieldName] = data.minLengthSeconds;
  }
  if (data.maxLengthSeconds !== undefined) {
    queryParams[maxLengthSecondsFieldName] = data.maxLengthSeconds;
  }

  const res = await axios.get(generateUrlWithQueryParams('/api/search/podcasts', queryParams)) as any;
  return {...res.data, hits: res.data.hits.map(deserializeShowInfo)};
};

export const getPodcastRssUrl = (data: {
  query?: string,
  tags?: string[],
  minLengthSeconds: number | undefined,
  maxLengthSeconds: number | undefined
}) => {
  const queryParams: {[key: string]: string | number} = {};
  if (data.query && data.query.length) {
    queryParams[queryFieldName] = data.query;
  }
  if (data.tags && data.tags.length) {
    queryParams[tagsFieldName] = data.tags.join(',');
  }
  if (data.minLengthSeconds !== undefined) {
    queryParams[minLengthSecondsFieldName] = data.minLengthSeconds;
  }
  if (data.maxLengthSeconds !== undefined) {
    queryParams[maxLengthSecondsFieldName] = data.maxLengthSeconds;
  }

  return encodeURI(generateUrlWithQueryParams('https://fdr-finder.tommyvolk.com/api/search/podcasts/rss', queryParams));
}

export const getFilteredTagsWithCounts =
async (data: {
  query?: string,
  limit?: number,
  offset?: number,
  tags?: string[],
  minLengthSeconds: number | undefined,
  maxLengthSeconds: number | undefined,
  filter?: string
}): Promise<{tags: {tag: string, count: number}[], remainingTagCount: number}> => {
  const queryParams: {[key: string]: string | number} = {};
  if (data.query && data.query.length) {
    queryParams[queryFieldName] = data.query;
  }
  if (data.limit !== undefined) {
    queryParams[limitFieldName] = data.limit;
  }
  if (data.offset !== undefined) {
    queryParams[offsetFieldName] = data.offset;
  }
  if (data.tags && data.tags.length) {
    queryParams[tagsFieldName] = data.tags.join(',');
  }
  if (data.minLengthSeconds !== undefined) {
    queryParams[minLengthSecondsFieldName] = data.minLengthSeconds;
  }
  if (data.maxLengthSeconds !== undefined) {
    queryParams[maxLengthSecondsFieldName] = data.maxLengthSeconds;
  }
  if (data.filter && data.filter.length) {
    queryParams[filterFieldName] = data.filter;
  }

  return (await axios.get(generateUrlWithQueryParams('/api/filteredTagsWithCounts', queryParams))).data as any;
}

export const generateUrlWithQueryParams =
(baseUrl: string, queryParams: {[key: string]: string | number | undefined}) => {
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