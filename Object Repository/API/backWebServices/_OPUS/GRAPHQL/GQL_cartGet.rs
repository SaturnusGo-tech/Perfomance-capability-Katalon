<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GQL_cartGet</name>
   <tag></tag>
   <elementGuidId>7e46d2e6-5d06-4149-ab07-a3aa490d6674</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;operationName\&quot;:\&quot;GetShortCart\&quot;,\&quot;variables\&quot;:{\&quot;storeId\&quot;:\&quot;${storeId}\&quot;,\&quot;userId\&quot;:\&quot;${userId}\&quot;,\&quot;cultureName\&quot;:\&quot;en-US\&quot;,\&quot;currencyCode\&quot;:\&quot;USD\&quot;},\&quot;query\&quot;:\&quot;query GetShortCart($storeId: String!, $userId: String!, $currencyCode: String!, $cultureName: String) {\\n  cart(\\n    storeId: $storeId\\n    userId: $userId\\n    currencyCode: $currencyCode\\n    cultureName: $cultureName\\n  ) {\\n    ...shortCartFields\\n  }\\n}\\n\\nfragment shortCartFields on CartType {\\n  id\\n  itemsQuantity\\n  items {\\n    id\\n    sku\\n    quantity\\n    productId\\n    extendedPrice {\\n      amount\\n    }\\n  }\\n}\\n\&quot;}&quot;,
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
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.token}</value>
      <webElementGuid>776c4bc5-779d-4f0d-beb8-c96a5e92805e</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>877818db-82bb-4530-836b-27ece9574709</id>
      <masked>false</masked>
      <name>storeId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>144f89c3-ccc1-4f82-9426-7e558d031858</id>
      <masked>false</masked>
      <name>userId</name>
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
