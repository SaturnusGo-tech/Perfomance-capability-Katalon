<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>userGet</name>
   <tag></tag>
   <elementGuidId>fff57ee4-f81c-4ba9-b06e-5db541ea2e93</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;operationName\&quot;: \&quot;GetMe\&quot;,\n\t\&quot;variables\&quot;: {},\n\t\&quot;query\&quot;: \&quot;query GetMe($after: String, $first: Int, $sort: String) {\\n  me {\\n    id\\n    memberId\\n    userName\\n    email\\n    emailConfirmed\\n    photoUrl\\n    phoneNumber\\n    permissions\\n    isAdministrator\\n    passwordExpired\\n    passwordExpiryInDays\\n    forcePasswordChange\\n    lockedState\\n    contact {\\n      firstName\\n      lastName\\n      fullName\\n      organizationId\\n      organizations(after: $after, first: $first, sort: $sort) {\\n        items {\\n          id\\n          name\\n        }\\n        items {\\n          status\\n          dynamicProperties {\\n            name\\n            value\\n          }\\n          accountsPayableContact {\\n            address {\\n              line1\\n              line2\\n              city\\n              countryCode\\n              countryName\\n              regionId\\n              regionName\\n              postalCode\\n            }\\n            email\\n            firstName\\n            lastName\\n            phone\\n            fullName\\n          }\\n        }\\n      }\\n      dynamicProperties {\\n        name\\n        value\\n      }\\n    }\\n    operator {\\n      userName\\n      contact {\\n        fullName\\n      }\\n    }\\n  }\\n}\&quot;\n}&quot;,
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
      <webElementGuid>0a918abf-dfef-4e14-b1b4-35a4fe3e390f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${GlobalVariable.authorizationCookie}</value>
      <webElementGuid>1645ec16-ce68-4b14-bb8a-2917baefa44b</webElementGuid>
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
      <defaultValue>''</defaultValue>
      <description>Authorization cookie. this variable is set at header section</description>
      <id>1d2898e5-e918-4ebc-aa07-d27c47b4dbb1</id>
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
