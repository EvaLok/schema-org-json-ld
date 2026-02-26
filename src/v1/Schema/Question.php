<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Question extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Question';

	public function __construct(
		public string $name,
		public null|Answer $acceptedAnswer = null,
		/** @var Answer[] $suggestedAnswer */
		public null|array $suggestedAnswer = null,
		public null|int $answerCount = null,
		public null|string $text = null,
		public null|int $upvoteCount = null,
		public null|Person|Organization $author = null,
		public null|string $datePublished = null,
		public null|string $dateModified = null,
		public null|string $eduQuestionType = null,
	) {}
}
