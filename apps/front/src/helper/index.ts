export const AreWeHome = () =>
  window.location.pathname === '/' ? true : false;

export const getMediaApiUrl = (path: string) => {
  return `${import.meta.env.VITE_API_URL as string}/api/media?path=${path}`;
};
