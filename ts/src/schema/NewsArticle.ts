import { Article } from "./Article.js";
import type { ArticleOptions } from "./Article.js";

export class NewsArticle extends Article {
	static readonly schemaType = "NewsArticle";

	constructor(options: ArticleOptions) {
		super(options);
	}
}
