<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class SolveMathAction extends TypedSchema {
	public const A_SCHEMA_TYPE = 'SolveMathAction';
	public const PROPERTY_MAP = [
		'mathExpressionInput' => 'mathExpression-input',
	];

	public function __construct(
		public string $target,
		public string $mathExpressionInput,
		public null|string|array $eduQuestionType = null,
	) {}
}
