<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Enum;

enum FulfillmentTypeEnumeration: string {
	case FulfillmentTypeDelivery = 'https://schema.org/FulfillmentTypeDelivery';
	case FulfillmentTypeCollectionPoint = 'https://schema.org/FulfillmentTypeCollectionPoint';
}
