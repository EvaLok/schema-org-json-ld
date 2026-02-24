<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class DiscussionForumPosting extends TypedSchema {
	public const A_SCHEMA_TYPE = 'DiscussionForumPosting';

	public function __construct(
		public Person|Organization $author,
		public string $datePublished,
		public string $text,
		public null|string $headline = null,
		public null|string $url = null,
		public null|string $dateModified = null,
		public null|ImageObject $image = null,
		public null|VideoObject $video = null,
		/** @var Comment[] $comment */
		public null|array $comment = null,
		public null|InteractionCounter $interactionStatistic = null,
		public null|string $isPartOf = null,
		public null|string $sharedContent = null,
		public null|string $creativeWorkStatus = null,
		public null|string $mainEntityOfPage = null,
	) {}
}
