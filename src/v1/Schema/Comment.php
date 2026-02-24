<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Comment extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Comment';

	public function __construct(
		public string $text,
		public null|Person|Organization $author = null,
		public null|string $datePublished = null,
	) {}
}
