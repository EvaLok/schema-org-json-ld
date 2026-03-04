import { Article } from "./Article.js";
import type { ArticleOptions } from "./Article.js";

export interface NewsArticleOptions extends ArticleOptions {}

export class NewsArticle extends Article {
	static readonly schemaType = "NewsArticle";
}
