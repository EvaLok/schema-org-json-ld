<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Comment extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Comment';

	public function __construct(
		public string $text,
		public null|Person|Organization $author = null,
		public null|string $datePublished = null,
		public null|string $url = null,
		public null|string $dateModified = null,
		public null|ImageObject $image = null,
		public null|VideoObject $video = null,
		/** @var Comment[] $comment */
		public null|array $comment = null,
		/** @var InteractionCounter|InteractionCounter[]|null $interactionStatistic */
		public null|InteractionCounter|array $interactionStatistic = null,
		public null|string $sharedContent = null,
		public null|string $creativeWorkStatus = null,
	) {}
}
