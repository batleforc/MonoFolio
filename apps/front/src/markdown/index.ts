export enum BlockType {
  string = 'string',
  title = 'title',
  startEndCodeBlock = 'startEndCodeBlock',
  checkbox = 'checkbox',
  list = 'list',
}

export interface additionalValue {
  code?: string;
  checked?: string;
  indent?: number;
}

export type Block = {
  type: BlockType;
  value: string;
  additionalValue?: additionalValue;
};

export class TitleBlock {
  type: BlockType = BlockType.title;
  level = 0;
  value = '';
  subBlocks: Block[] = [];
  subTitleBlock: TitleBlock[] = [];

  constructor(value: string) {
    this.value = value.replace(new RegExp('#{0,}'), '').trim();
    this.level = value.split('').filter((char: string) => char === '#').length;
  }

  appendTitleBlock = (param: TitleBlock) => {
    if (param.level - 1 === this.level) {
      this.subTitleBlock.push(param);
    } else if (this.subTitleBlock.length > 0) {
      this.subTitleBlock[this.subTitleBlock.length - 1].appendTitleBlock(param);
    } else {
      this.subTitleBlock.push(param);
    }
  };
  appendBlock = (block: Block) => {
    if (this.subTitleBlock.length > 0) {
      this.subTitleBlock[this.subTitleBlock.length - 1].appendBlock(block);
    } else {
      this.subBlocks.push(block);
    }
  };
}

export const transformContent = (content: string): TitleBlock[] => {
  const titleBlocks: TitleBlock[] = [];
  const lines = content.split('\n');
  let codeBlock: string[] = [];
  lines.forEach((line) => {
    // Title
    if (line.trim() === '') return;
    if (line.startsWith('#')) {
      const titleBlock = new TitleBlock(line);
      if (
        titleBlocks.length === 0 ||
        titleBlocks[titleBlocks.length - 1].level === titleBlock.level
      )
        titleBlocks.push(titleBlock);
      if (titleBlocks[titleBlocks.length - 1].level < titleBlock.level) {
        titleBlocks[titleBlocks.length - 1].appendTitleBlock(titleBlock);
      }
    } else {
      const block: Block = {
        type: BlockType.string,
        value: line,
      };
      if (line.startsWith('```')) {
        if (codeBlock.length > 0) {
          block.type = BlockType.startEndCodeBlock;
          block.additionalValue = {};
          block.additionalValue.code = codeBlock[0].replace('```', '');
          codeBlock.shift();
          block.value = codeBlock.join('\n');
          codeBlock = [];
        } else codeBlock.push(line);
      } else if (codeBlock.length > 0) {
        codeBlock.push(line);
      } else if (
        new RegExp('\\s{0,}-\\s\\[[x,X,\\s]\\]\\s[A-z]{0,}').test(line) ||
        line.startsWith('- [ ]') ||
        line.startsWith('- [x]')
      ) {
        block.type = BlockType.checkbox;
        block.additionalValue = {};
        block.additionalValue.indent = line.split('- ')[0].length + 1;
        if (line.startsWith('- [x]')) block.additionalValue.checked = 'checked';
        else block.additionalValue.checked = 'unchecked';
        block.value = line.replace('- [ ]', '').replace('- [x]', '').trim();
      } else if (new RegExp('\\s{0,}-\\s[A-z]{0,}').test(line)) {
        block.type = BlockType.list;
        block.value = line.replace('- ', '').trim();
        block.additionalValue = {};
        block.additionalValue.indent = line.split('- ')[0].length + 1;
      }
      if (titleBlocks.length === 0) {
        titleBlocks.push(new TitleBlock(''));
      }
      if (codeBlock.length === 0)
        titleBlocks[titleBlocks.length - 1].appendBlock(block);
    }
  });
  return titleBlocks;
};
