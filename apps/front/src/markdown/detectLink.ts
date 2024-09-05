export type Link = {
  url: string;
  text: string;
};

export const detectLink = (text: string) => {
  const textUrlArray = [];
  const urlMdRegex = /\[(\?'text'[^\]]+)\]\((\?'url'[^)]+)\)/g;
  while (urlMdRegex.exec(text)) {
    textUrlArray.push({
      text: RegExp.$1,
      url: RegExp.$2,
    });
  }
  const urlMdRegex2 = /<(\?'text'[^\]]+)>/g;
  // eslint-disable-next-line no-constant-condition
  const matched = urlMdRegex2.exec(text);
  if (!matched) return;
  textUrlArray.push({
    text: matched[1],
    url: matched[1],
  });
  const splitText = text.split(`<${matched[1]}>`);
  textUrlArray.push(splitText[0]);

  text = splitText.slice(1).join('');
};
