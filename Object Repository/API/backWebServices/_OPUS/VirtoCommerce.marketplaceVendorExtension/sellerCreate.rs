<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>sellerCreate</name>
   <tag></tag>
   <elementGuidId>f0a86cb9-7435-4e6a-9c26-c64d6ac0c5a9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;ownerDetails\&quot;: {\n\t\t\&quot;firstName\&quot;: \&quot;katalonVendorFirstName\&quot;,\n\t\t\&quot;lastName\&quot;: \&quot;katalonVendorLastName\&quot;,\n\t\t\&quot;email\&quot;: \&quot;${email}\&quot;\n\t},\n\t\&quot;sellerName\&quot;: \&quot;${sellerName}\&quot;,\n\t\&quot;commissionFeeId\&quot;: \&quot;${comissionFeeId}\&quot;\n}&quot;,
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
      <webElementGuid>96c47380-2fe5-460d-ade5-bfade71588f2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_key</name>
      <type>Main</type>
      <value>${GlobalVariable.api_key}</value>
      <webElementGuid>01eae408-7219-4a90-9ccb-1e583494d6c6</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/api/vcmp/security/seller/create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'katalonVendor'</defaultValue>
      <description>name of the created seller</description>
      <id>1edbeee0-d373-4b8b-8682-d0cdc480c7fa</id>
      <masked>false</masked>
      <name>sellerName</name>
   </variables>
   <variables>
      <defaultValue>'d541e541-6a10-4882-b8c1-e051e9547f4f'</defaultValue>
      <description>id of the related fee</description>
      <id>a5a9c355-4d13-49a8-a9e9-e84a0d0d9307</id>
      <masked>false</masked>
      <name>comissionFeeId</name>
   </variables>
   <variables>
      <defaultValue>'evgtest@list.ru'</defaultValue>
      <description>email of the created vendor</description>
      <id>a53e0846-6062-4dc7-b574-1db33d2814f4</id>
      <masked>false</masked>
      <name>email</name>
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
WS.verifyResponseStatusCode(response, 200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
