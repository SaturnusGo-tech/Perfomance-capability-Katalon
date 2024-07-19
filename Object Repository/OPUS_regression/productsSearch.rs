<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>productsSearch</name>
   <tag></tag>
   <elementGuidId>d3fa7270-604b-40df-b678-8f5721e5b807</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;operationName\&quot;: \&quot;SearchProducts\&quot;,\n  \&quot;variables\&quot;: {\n    \&quot;storeId\&quot;: \&quot;opus\&quot;,\n    \&quot;userId\&quot;: \&quot;${userId}\&quot;,\n    \&quot;cultureName\&quot;: \&quot;en-US\&quot;,\n    \&quot;currencyCode\&quot;: \&quot;USD\&quot;,\n    \&quot;sort\&quot;: \&quot;\&quot;,\n    \&quot;withFacets\&quot;: true,\n    \&quot;withImages\&quot;: true,\n    \&quot;withProductOffers\&quot;: false,\n    \&quot;query\&quot;: \&quot;\&quot;,\n    \&quot;filter\&quot;: \&quot;category.subtree:21057e9c-14df-48c1-be58-0c59a4f38f06 \\\&quot;SupplierOuterId\\\&quot;:\\\&quot;${supplierOuterId}\\\&quot;\&quot;,\n    \&quot;first\&quot;: 100,\n    \&quot;after\&quot;: \&quot;0\&quot;\n  },\n  \&quot;query\&quot;: \&quot;query SearchProducts($storeId: String!, $userId: String!, $currencyCode: String!, $cultureName: String, $filter: String, $after: String, $first: Int, $sort: String, $query: String, $fuzzy: Boolean, $fuzzyLevel: Int, $productIds: [String], $withFacets: Boolean!, $withImages: Boolean!, $withProductOffers: Boolean!) {\\n  products(\\n    storeId: $storeId\\n    userId: $userId\\n    after: $after\\n    first: $first\\n    filter: $filter\\n    sort: $sort\\n    currencyCode: $currencyCode\\n    cultureName: $cultureName\\n    query: $query\\n    fuzzy: $fuzzy\\n    fuzzyLevel: $fuzzyLevel\\n    productIds: $productIds\\n  ) {\\n    totalCount\\n    items {\\n      name\\n      id\\n      code\\n      minQuantity\\n      maxQuantity\\n      inWishlist\\n      productType\\n      vendor {\\n        id\\n        name\\n        supplier {\\n          contractNumber\\n        }\\n      }\\n      hasVariations\\n      slug\\n      outline\\n      imgSrc\\n      images @include(if: $withImages) {\\n        url\\n        group\\n      }\\n      description(type: \\\&quot;QuickReview\\\&quot;) {\\n        content\\n      }\\n      availabilityData @include(if: $withProductOffers) {\\n        ...availabilityDataFields\\n      }\\n      price @include(if: $withProductOffers) {\\n        actual {\\n          ...moneyFields\\n        }\\n        discountAmount {\\n          amount\\n          formattedAmount\\n        }\\n        sale {\\n          amount\\n          formattedAmount\\n        }\\n        list {\\n          ...moneyFields\\n        }\\n        discountPercent\\n      }\\n      keyProperties {\\n        ...propertyFields\\n      }\\n    }\\n    term_facets @include(if: $withFacets) {\\n      name\\n      label\\n      terms {\\n        label\\n        term\\n        count\\n        isSelected\\n      }\\n    }\\n    range_facets @include(if: $withFacets) {\\n      name\\n      label\\n      ranges {\\n        label\\n        count\\n        from\\n        to\\n        includeFrom\\n        includeTo\\n        isSelected\\n      }\\n    }\\n  }\\n}\\n\\nfragment propertyFields on Property {\\n  name\\n  value\\n  type\\n  hidden\\n  valueType\\n  label\\n}\\n\\nfragment availabilityDataFields on AvailabilityData {\\n  isActive\\n  isAvailable\\n  isBuyable\\n  isInStock\\n  availableQuantity\\n  isEstimated\\n}\\n\\nfragment moneyFields on MoneyType {\\n  amount\\n  formattedAmount\\n  formattedAmountWithoutCurrency\\n  currency {\\n    ...currencyFields\\n  }\\n}\\n\\nfragment currencyFields on CurrencyType {\\n  code\\n  symbol\\n}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c0141233-e5b2-4c87-ad29-33a320387433</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${GlobalVariable.authorizationCookie}</value>
      <webElementGuid>776c4bc5-779d-4f0d-beb8-c96a5e92805e</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/xapi/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'ca8ba2ca-1f4b-4a19-81c4-36a3591e2fc5'</defaultValue>
      <description></description>
      <id>144f89c3-ccc1-4f82-9426-7e558d031858</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>'300'</defaultValue>
      <description>outerid of the selected supplier</description>
      <id>e3e9c7b2-31da-411c-9c4a-677eb17da266</id>
      <masked>false</masked>
      <name>supplierOuterId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9ca185ac-47a1-421c-8585-7c56e74cb8ee</id>
      <masked>false</masked>
      <name>Cookie</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonOutput
import com.kms.katalon.core.util.KeywordUtil


ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
def prettyJson = JsonOutput.prettyPrint(response.getResponseBodyContent())
KeywordUtil.logInfo(prettyJson)
WS.verifyResponseStatusCode(response, 200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
