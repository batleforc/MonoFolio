export type TextSpec = {
  start: number;
  stop: number;
  isLink: boolean;
  url?: string;
  isBold: boolean;
  isItalic: boolean;
  isCode: boolean;
};

export const detectTextSpec = (text: string) => {
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
  while (true) {
    text.search(urlMdRegex2);
    const matched = urlMdRegex2.exec(text);
    if (!matched) break;
    const splitText = text.split(`<${matched[1]}>`);
    textUrlArray;

    text = splitText.slice(1).join('');
  }
};
