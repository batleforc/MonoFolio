export enum BlockType {
  string = 'string',
  title = 'title',
  startEndCodeBlock = 'startEndCodeBlock',
}

export type Block = {
  type: BlockType;
  value: string;
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
        block.type = BlockType.startEndCodeBlock;
      }
      if (titleBlocks.length === 0) {
        titleBlocks.push(new TitleBlock(''));
      }
      titleBlocks[titleBlocks.length - 1].appendBlock(block);
    }
  });
  console.log(titleBlocks);
  return titleBlocks;
};
