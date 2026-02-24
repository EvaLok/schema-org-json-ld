<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Question extends TypedSchema {

	const A_SCHEMA_TYPE = 'Question';

	public function __construct(
		public string $name,
		public Answer $acceptedAnswer,
	) {

	}
}
