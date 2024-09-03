export enum BlockType {
  string = 'string',
  title = 'title',
  startEndCodeBlock = 'startEndCodeBlock',
  checkbox = 'checkbox',
}

export type Block = {
  type: BlockType;
  value: string;
  additionalValue?: string;
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
          block.additionalValue = codeBlock[0].replace('```', '');
          codeBlock.shift();
          block.value = codeBlock.join('\n');
          codeBlock = [];
        } else codeBlock.push(line);
      } else if (codeBlock.length > 0) {
        codeBlock.push(line);
      } else if (line.startsWith('- [ ]') || line.startsWith('- [x]')) {
        block.type = BlockType.checkbox;
        if (line.startsWith('- [x]')) block.additionalValue = 'checked';
        else block.additionalValue = 'unchecked';
        block.value = line.replace('- [ ]', '').replace('- [x]', '').trim();
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
