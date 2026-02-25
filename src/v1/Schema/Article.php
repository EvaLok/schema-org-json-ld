<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Article extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Article';

	public function __construct(
		public string $headline,
		/** @var array<Person|Organization> $author */
		public null|Person|Organization|array $author = null,
		public null|string $datePublished = null,
		public null|string $dateModified = null,
		/** @var array<string|ImageObject> $image */
		public null|array $image = null,
		public null|string $description = null,
		public null|Organization $publisher = null,
		public null|SpeakableSpecification $speakable = null,
		public null|bool $isAccessibleForFree = null,
		/** @var WebPageElement[]|WebPageElement|null $hasPart */
		public null|WebPageElement|array $hasPart = null,
	) {}
}
