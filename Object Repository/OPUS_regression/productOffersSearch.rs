<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>productOffersSearch</name>
   <tag></tag>
   <elementGuidId>a40cc2b4-605f-43d6-8336-552d5c2fe6c9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;operationName\&quot;: \&quot;SearchProductsOffers\&quot;,\n\t\&quot;variables\&quot;: {\n\t\t\&quot;storeId\&quot;: \&quot;opus\&quot;,\n\t\t\&quot;userId\&quot;: \&quot;${userId}\&quot;,\n\t\t\&quot;cultureName\&quot;: \&quot;en-US\&quot;,\n\t\t\&quot;currencyCode\&quot;: \&quot;USD\&quot;,\n\t\t\&quot;sort\&quot;: \&quot;\&quot;,\n\t\t\&quot;query\&quot;: \&quot;\&quot;,\n\t\t\&quot;filter\&quot;: \&quot;category.subtree:21057e9c-14df-48c1-be58-0c59a4f38f06/${categoryId} \\\&quot;SupplierOuterId\\\&quot;:\\\&quot;${supplierOuterId}\\\&quot;\&quot;,\n\t\t\&quot;first\&quot;: 16,\n\t\t\&quot;after\&quot;: \&quot;0\&quot;\n\t},\n\t\&quot;query\&quot;: \&quot;query SearchProductsOffers($storeId: String!, $userId: String!, $currencyCode: String!, $cultureName: String, $filter: String, $after: String, $first: Int, $sort: String, $query: String, $fuzzy: Boolean, $fuzzyLevel: Int, $productIds: [String]) {\\n  products(\\n    storeId: $storeId\\n    userId: $userId\\n    after: $after\\n    first: $first\\n    filter: $filter\\n    sort: $sort\\n    currencyCode: $currencyCode\\n    cultureName: $cultureName\\n    query: $query\\n    fuzzy: $fuzzy\\n    fuzzyLevel: $fuzzyLevel\\n    productIds: $productIds\\n  ) {\\n    items {\\n      id\\n      availabilityData {\\n        ...availabilityDataFields\\n      }\\n      price {\\n        actual {\\n          amount\\n          formattedAmount\\n          formattedAmountWithoutCurrency\\n          currency {\\n            symbol\\n          }\\n        }\\n        discountAmount {\\n          amount\\n          formattedAmount\\n        }\\n        sale {\\n          amount\\n          formattedAmount\\n        }\\n        list {\\n          amount\\n          formattedAmount\\n          formattedAmountWithoutCurrency\\n          currency {\\n            symbol\\n          }\\n        }\\n        discountPercent\\n      }\\n    }\\n  }\\n}\\n\\nfragment availabilityDataFields on AvailabilityData {\\n  isActive\\n  isAvailable\\n  isBuyable\\n  isInStock\\n  availableQuantity\\n  isEstimated\\n}\&quot;\n}&quot;,
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
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9ca185ac-47a1-421c-8585-7c56e74cb8ee</id>
      <masked>false</masked>
      <name>Cookie</name>
   </variables>
   <variables>
      <defaultValue>'300'</defaultValue>
      <description></description>
      <id>4370c3b7-a0c7-4de8-a3fe-6d807744df17</id>
      <masked>false</masked>
      <name>supplierOuterId</name>
   </variables>
   <variables>
      <defaultValue>'0c67db77-0c3c-4a43-ac98-ec189272ee58'</defaultValue>
      <description></description>
      <id>d68c0253-a562-45f8-a1de-55968dbb4ed8</id>
      <masked>false</masked>
      <name>categoryId</name>
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
