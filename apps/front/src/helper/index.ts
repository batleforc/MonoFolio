export const AreWeHome = () =>
  window.location.pathname === '/' ? true : false;

export const getMediaApiUrl = (path: string) => {
  return `${import.meta.env.VITE_API_URL as string}/api/media?path=${path}`;
};

export enum MediaType {
  Ico = 'ico#',
  ApiMedia = 'media#',
  Url = 'url#',
  Http = 'http',
  None = 'none',
}

export const getMediaUrl = (path: string) => {
  if (path.startsWith('http')) {
    return {
      type: MediaType.Http,
      url: path,
    };
  }
  if (path.startsWith('url#')) {
    return {
      type: MediaType.Url,
      url: path.replace('url#', ''),
    };
  }
  if (path.startsWith('media#')) {
    return {
      type: MediaType.ApiMedia,
      url: getMediaApiUrl(path.replace('media#', '')),
    };
  }
  if (path.startsWith('ico#')) {
    return {
      type: MediaType.Ico,
      url: path.replace('ico#', ''),
    };
  }
  return {
    type: MediaType.None,
    url: path,
  };
};

export const formatString = (str: string, length = 35) => {
  let target = str.substring(0, length);
  if (target.length === length) {
    target += '...';
  }
  return target;
};
