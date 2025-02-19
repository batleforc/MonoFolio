// This file is auto-generated by @hey-api/openapi-ts

/**
 * Alignment kind in the markdown AST.
 */
export type AlignKind = 'Left' | 'Right' | 'Center' | 'None';

/**
 * Attribute content in the markdown AST.
 */
export type AttributeContent = {
    Expression: MdxJsxExpressionAttribute;
} | {
    Property: MdxJsxAttribute;
};

/**
 * Attribute value in the markdown AST.
 */
export type AttributeValue = {
    Expression: AttributeValueExpression;
} | {
    Literal: string;
};

/**
 * Attribute value expression in the markdown AST.
 */
export type AttributeValueExpression = {
    value: string;
};

/**
 * Blockquote node in the markdown AST.
 */
export type Blockquote = {
    children: Array<Node>;
};

/**
 * Timeline of the blog space, contain the short representation of the pages.
 */
export type BlogTimeline = {
    pages: {
        [key: string]: PageShort;
    };
};

/**
 * Break node in the markdown AST.
 */
export type Break = {
    [key: string]: unknown;
};

/**
 * Code node in the markdown AST.
 */
export type Code = {
    lang?: string | null;
    meta?: string | null;
    value: string;
};

/**
 * Definition node in the markdown AST.
 */
export type Definition = {
    identifier: string;
    label?: string | null;
    title?: string | null;
    url: string;
};

/**
 * Delete node in the markdown AST.
 */
export type Delete = {
    children: Array<Node>;
};

/**
 * The recursive structure of the sidebar containing the pages and sub-categories of the documentation.
 */
export type DocCategory = {
    has_index: boolean;
    name: string;
    pages: Array<PageShort>;
    sub_categories: Array<DocCategory>;
};

/**
 * Header of a markdown file.
 */
export type DocHeader = {
    date: string;
    description?: string | null;
    image?: string | null;
    links?: Array<DocHeaderLink>;
    spec?: DocHeaderSpec;
    tags?: Array<string>;
    techno?: Array<string>;
    title: string;
    weight?: number;
    writer?: DocHeaderWriter;
};

/**
 * Link present in the header of a markdown file.
 */
export type DocHeaderLink = {
    name: string;
    url: string;
};

/**
 * Specification of the header of a markdown file, include the information if a Header is a blog/project/doc.
 */
export type DocHeaderSpec = {
    blog?: boolean;
    doc?: boolean;
    project?: boolean;
};

/**
 * Writer present in the header of a markdown file.
 */
export type DocHeaderWriter = {
    avatar: string;
    name: string;
    url: string;
};

/**
 * Emphasis node in the markdown AST.
 */
export type Emphasis = {
    children: Array<Node>;
};

/**
 * Footnote definition node in the markdown AST.
 */
export type FootnoteDefinition = {
    children: Array<Node>;
    identifier: string;
    label?: string | null;
};

/**
 * Footnote reference node in the markdown AST.
 */
export type FootnoteReference = {
    identifier: string;
    label?: string | null;
};

/**
 * Heading node in the markdown AST.
 */
export type Heading = {
    children: Array<Node>;
    depth: number;
};

/**
 * Content of the home page.
 */
export type HomeContent = {
    coverTitle: Array<string>;
    cvUrl: string;
    history: Array<HomeHistory>;
    name: string;
    presentation: string;
    shortDescription: string;
    url: Array<HomeUrl>;
};

/**
 * History of the home page, contain a part of the history of Max Batleforc.
 */
export type HomeHistory = {
    date: string;
    description: string;
    imgUrl: string;
    lieux: string;
    title: string;
    url?: Array<HomeHistoryUrl> | null;
    weight: number;
};

/**
 * Url that should be present on the history.
 */
export type HomeHistoryUrl = {
    name: string;
    url: string;
};

/**
 * Url that should be present on the home page.
 */
export type HomeUrl = {
    imgUrl: string;
    name: string;
    primaire: boolean;
    url: string;
};

/**
 * HTML node in the markdown AST.
 */
export type Html = {
    value: string;
};

/**
 * Image node in the markdown AST.
 */
export type Image = {
    alt: string;
    title?: string | null;
    url: string;
};

/**
 * Image reference node in the markdown AST.
 */
export type ImageReference = {
    alt: string;
    identifier: string;
    label?: string | null;
    referenceType: ReferenceKind;
};

/**
 * Inline code node in the markdown AST.
 */
export type InlineCode = {
    value: string;
};

/**
 * Inline math node in the markdown AST.
 */
export type InlineMath = {
    value: string;
};

/**
 * Link node in the markdown AST.
 */
export type Link = {
    children: Array<Node>;
    title?: string | null;
    url: string;
};

/**
 * Link reference node in the markdown AST.
 */
export type LinkReference = {
    children: Array<Node>;
    identifier: string;
    label?: string | null;
    referenceType: ReferenceKind;
};

/**
 * List node in the markdown AST.
 */
export type List = {
    children: Array<Node>;
    ordered: boolean;
    spread: boolean;
    start?: number | null;
};

/**
 * List item node in the markdown AST.
 */
export type ListItem = {
    checked?: boolean | null;
    children: Array<Node>;
    spread: boolean;
};

/**
 * Math node in the markdown AST.
 */
export type _Math = {
    meta?: string | null;
    value: string;
};

/**
 * MDX flow expression node in the markdown AST.
 */
export type MdxFlowExpression = {
    value: string;
};

/**
 * JSX attribute in the markdown AST.
 */
export type MdxJsxAttribute = {
    name: string;
    value?: null | AttributeValue;
};

/**
 * JSX expression attribute in the markdown AST.
 */
export type MdxJsxExpressionAttribute = {
    /**
     * Value.
     */
    value: string;
};

/**
 * JSX flow element node in the markdown AST.
 */
export type MdxJsxFlowElement = {
    attributes: Array<AttributeContent>;
    children: Array<Node>;
    name?: string | null;
};

/**
 * JSX text element node in the markdown AST.
 */
export type MdxJsxTextElement = {
    attributes: Array<AttributeContent>;
    children: Array<Node>;
    name?: string | null;
};

/**
 * MDX text expression node in the markdown AST.
 */
export type MdxTextExpression = {
    value: string;
};

/**
 * ESM node in the markdown AST.
 */
export type MdxjsEsm = {
    value: string;
};

/**
 * Main kind of node in the markdown AST.
 */
export type Node = {
    Root: Root;
} | {
    Blockquote: Blockquote;
} | {
    FootnoteDefinition: FootnoteDefinition;
} | {
    MdxJsxFlowElement: MdxJsxFlowElement;
} | {
    List: List;
} | {
    MdxjsEsm: MdxjsEsm;
} | {
    Toml: Toml;
} | {
    Yaml: Yaml;
} | {
    Break: Break;
} | {
    InlineCode: InlineCode;
} | {
    InlineMath: InlineMath;
} | {
    Delete: Delete;
} | {
    Emphasis: Emphasis;
} | {
    MdxTextExpression: MdxTextExpression;
} | {
    FootnoteReference: FootnoteReference;
} | {
    Html: Html;
} | {
    Image: Image;
} | {
    ImageReference: ImageReference;
} | {
    MdxJsxTextElement: MdxJsxTextElement;
} | {
    Link: Link;
} | {
    LinkReference: LinkReference;
} | {
    Strong: Strong;
} | {
    Text: Text;
} | {
    Code: Code;
} | {
    Math: _Math;
} | {
    MdxFlowExpression: MdxFlowExpression;
} | {
    Heading: Heading;
} | {
    Table: Table;
} | {
    ThematicBreak: ThematicBreak;
} | {
    TableRow: TableRow;
} | {
    TableCell: TableCell;
} | {
    ListItem: ListItem;
} | {
    Definition: Definition;
} | {
    Paragraph: Paragraph;
};

/**
 * A page containing the content, metadata and name of the page.
 */
export type Page = {
    content: string;
    metadata: DocHeader;
    name: string;
};

/**
 * Short representation of a page, containing only the metadata and the path to the page.
 */
export type PageShort = {
    metadata: DocHeader;
    name: string;
    path: string;
};

/**
 * V2 of the page struct, containing the metadata, path and commonmark content.
 */
export type PageV2 = {
    metadata: DocHeader;
    name: string;
    parsed_content: Node;
};

/**
 * Paragraph node in the markdown AST.
 */
export type Paragraph = {
    children: Array<Node>;
};

/**
 * List of projects, contain the short representation of the pages, mainly aimed at the Home screen.
 */
export type ProjectList = {
    projects: {
        [key: string]: PageShort;
    };
};

/**
 * Reference kind in the markdown AST.
 */
export type ReferenceKind = 'Shortcut' | 'Collapsed' | 'Full';

/**
 * Root node of the markdown AST.
 */
export type Root = {
    children: Array<Node>;
};

/**
 * Strong node in the markdown AST.
 */
export type Strong = {
    children: Array<Node>;
};

/**
 * Table node in the markdown AST.
 */
export type Table = {
    align: Array<AlignKind>;
    children: Array<Node>;
};

/**
 * Table cell node in the markdown AST.
 */
export type TableCell = {
    children: Array<Node>;
};

/**
 * Table row node in the markdown AST.
 */
export type TableRow = {
    children: Array<Node>;
};

/**
 * Text node in the markdown AST.
 */
export type Text = {
    value: string;
};

/**
 * Thematic break node in the markdown AST.
 */
export type ThematicBreak = {
    [key: string]: unknown;
};

/**
 * TOML node in the markdown AST.
 */
export type Toml = {
    value: string;
};

/**
 * YAML node in the markdown AST.
 */
export type Yaml = {
    value: string;
};

export type GetTimelineData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/api/blog';
};

export type GetTimelineErrors = {
    /**
     * Internal server error.
     */
    500: unknown;
};

export type GetTimelineResponses = {
    /**
     * Content of the blog timeline ordered by date.
     */
    200: BlogTimeline;
};

export type GetTimelineResponse = GetTimelineResponses[keyof GetTimelineResponses];

export type GetDocSidebarData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/api/doc';
};

export type GetDocSidebarErrors = {
    /**
     * Internal server error.
     */
    500: unknown;
};

export type GetDocSidebarResponses = {
    /**
     * Doc category sidebar.
     */
    200: DocCategory;
};

export type GetDocSidebarResponse = GetDocSidebarResponses[keyof GetDocSidebarResponses];

export type GetHomeData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/api/home';
};

export type GetHomeErrors = {
    /**
     * Internal server error.
     */
    500: unknown;
};

export type GetHomeResponses = {
    /**
     * Content of the home page.
     */
    200: HomeContent;
};

export type GetHomeResponse = GetHomeResponses[keyof GetHomeResponses];

export type GetMediaData = {
    body?: never;
    path?: never;
    query: {
        /**
         * Path to the media asset.
         */
        path: string;
    };
    url: '/api/media';
};

export type GetMediaErrors = {
    /**
     * Page not found inside of the Media folder or path invalid.
     */
    404: unknown;
    /**
     * Internal server error.
     */
    500: unknown;
};

export type GetMediaResponses = {
    /**
     * Media content if found.
     */
    200: unknown;
};

export type GetPageData = {
    body?: never;
    path?: never;
    query: {
        /**
         * Path to the markdown page.
         */
        path: string;
    };
    url: '/api/page';
};

export type GetPageErrors = {
    /**
     * Page not found or path invalid.
     */
    404: unknown;
    /**
     * Internal server error.
     */
    500: unknown;
};

export type GetPageResponses = {
    /**
     * Content of a full page if found.
     */
    200: Page;
};

export type GetPageResponse = GetPageResponses[keyof GetPageResponses];

export type GetProjectListData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/api/projects';
};

export type GetProjectListErrors = {
    /**
     * Internal server error.
     */
    500: unknown;
};

export type GetProjectListResponses = {
    /**
     * Content of the Project List.
     */
    200: ProjectList;
};

export type GetProjectListResponse = GetProjectListResponses[keyof GetProjectListResponses];

export type GetPageV2Data = {
    body?: never;
    path?: never;
    query: {
        /**
         * Path to the markdown page.
         */
        path: string;
    };
    url: '/api/v2/page';
};

export type GetPageV2Errors = {
    /**
     * Page not found or path invalid.
     */
    404: unknown;
    /**
     * Internal server error.
     */
    500: unknown;
};

export type GetPageV2Responses = {
    /**
     * Content of a full page if found.
     */
    200: PageV2;
};

export type GetPageV2Response = GetPageV2Responses[keyof GetPageV2Responses];

export type ClientOptions = {
    baseURL: 'http://localhost:5437' | 'https://maxleriche.net' | 'https://beta.maxleriche.net' | (string & {});
};