<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>masterCatalogCreate</name>
   <tag></tag>
   <elementGuidId>4315b027-bc72-408e-8cf2-53e76cd3a9f1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;name\&quot;: \&quot;OPUS\&quot;,\n\t\&quot;isVirtual\&quot;: false,\n\t\&quot;outerId\&quot;: null,\n\t\&quot;defaultLanguage\&quot;: {\n\t\t\&quot;catalogId\&quot;: null,\n\t\t\&quot;isDefault\&quot;: true,\n\t\t\&quot;languageCode\&quot;: \&quot;en-US\&quot;,\n\t\t\&quot;id\&quot;: null\n\t},\n\t\&quot;languages\&quot;: [\n\t\t{\n\t\t\t\&quot;catalogId\&quot;: null,\n\t\t\t\&quot;isDefault\&quot;: true,\n\t\t\t\&quot;languageCode\&quot;: \&quot;en-US\&quot;,\n\t\t\t\&quot;id\&quot;: null\n\t\t}\n\t],\n\t\&quot;properties\&quot;: null,\n\t\&quot;createdDate\&quot;: \&quot;0001-01-01T00:00:00Z\&quot;,\n\t\&quot;modifiedDate\&quot;: null,\n\t\&quot;createdBy\&quot;: null,\n\t\&quot;modifiedBy\&quot;: null,\n\t\&quot;id\&quot;: \&quot;6f272f5b-a13c-46fb-ab5b-51aeb3e8e9e4\&quot;\n}&quot;,
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
      <webElementGuid>3e85119c-826a-4eac-882d-e46727470d0e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_key</name>
      <type>Main</type>
      <value>${GlobalVariable.api_key}</value>
      <webElementGuid>c76a6874-9873-4cf0-a3e4-3980bdf94ead</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/api/catalog/catalogs</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'TestCatalog'</defaultValue>
      <description>Name of the created catalog</description>
      <id>43a1ae75-290c-4d7e-88df-04a0c03bffda</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

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
