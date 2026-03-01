import { Article } from "./Article.js";
import type { ArticleOptions } from "./Article.js";

export class BlogPosting extends Article {
	static readonly schemaType = "BlogPosting";

	constructor(options: ArticleOptions) {
		super(options);
	}
}
