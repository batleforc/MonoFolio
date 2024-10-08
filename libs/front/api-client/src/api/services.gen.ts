// This file is auto-generated by @hey-api/openapi-ts

import {
  createClient,
  createConfig,
  type Options,
} from '@hey-api/client-axios';
import type {
  GetTimelineError,
  GetTimelineResponse,
  GetDocSidebarError,
  GetDocSidebarResponse,
  GetHomeError,
  GetHomeResponse,
  GetMediaData,
  GetMediaError,
  GetMediaResponse,
  GetPageData,
  GetPageError,
  GetPageResponse,
} from './types.gen';

export const client = createClient(createConfig());

/**
 * Get the blog timeline
 * Get blog timeline with minimal description of each article.
 */
export const getTimeline = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    GetTimelineResponse,
    GetTimelineError,
    ThrowOnError
  >({
    ...options,
    url: '/api/blog',
  });
};

/**
 * Get doc sidebar
 * Get doc sidebar with minimal description of each article.
 */
export const getDocSidebar = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    GetDocSidebarResponse,
    GetDocSidebarError,
    ThrowOnError
  >({
    ...options,
    url: '/api/doc',
  });
};

/**
 * Get home content
 * Get the content meant for the home page.
 */
export const getHome = <ThrowOnError extends boolean = false>(
  options?: Options<unknown, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    GetHomeResponse,
    GetHomeError,
    ThrowOnError
  >({
    ...options,
    url: '/api/home',
  });
};

/**
 * Get media from the media folder
 * Get the media asset from the media folder.
 */
export const getMedia = <ThrowOnError extends boolean = false>(
  options: Options<GetMediaData, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    GetMediaResponse,
    GetMediaError,
    ThrowOnError
  >({
    ...options,
    url: '/api/media',
  });
};

/**
 * Get a page content
 * Fetch page's content by page path in DbFolder.
 */
export const getPage = <ThrowOnError extends boolean = false>(
  options: Options<GetPageData, ThrowOnError>,
) => {
  return (options?.client ?? client).get<
    GetPageResponse,
    GetPageError,
    ThrowOnError
  >({
    ...options,
    url: '/api/page',
  });
};
