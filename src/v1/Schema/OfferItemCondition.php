<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum OfferItemCondition: string {
	case NewCondition = 'https://schema.org/NewCondition';
	case RefurbishedCondition = 'https://schema.org/RefurbishedCondition';
	case UsedCondition = 'https://schema.org/UsedCondition';
	case DamagedCondition = 'https://schema.org/DamagedCondition';

}
