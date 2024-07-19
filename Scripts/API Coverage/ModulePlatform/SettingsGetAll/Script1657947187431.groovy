import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import groovy.json.JsonSlurper as JsonSlurper

WebUI.comment('TEST CASE: Get info about all settings')

response = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/SettingsGetAll'))
//WS.verifyElementPropertyValue(response, '[0].name', "VirtoCommerce.Platform.Security.AccountTypes")
//WS.verifyElementPropertyValue(response, '[0].defaultValue', "Customer")
jsonResponse = response.getResponseBodyContent()
json = new JsonSlurper().parseText(jsonResponse)
targetName = 'VirtoCommerce.Platform.Security.AccountTypes'
targetObject = json.find { it.name == targetName }
assert targetObject.defaultValue == "Customer" 

