import { Article } from "./Article.js";
import type { ArticleOptions } from "./Article.js";

export interface BlogPostingOptions extends ArticleOptions {}

export class BlogPosting extends Article {
	static readonly schemaType = "BlogPosting";
}
