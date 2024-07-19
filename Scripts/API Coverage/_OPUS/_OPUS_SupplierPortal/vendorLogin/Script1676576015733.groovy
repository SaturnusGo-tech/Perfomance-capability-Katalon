import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable as GlobalVariable


WebUI.comment('TEST CASE: log in and verify')


//it is possible to just replace log/pass pair in the og auth by callTestCase
'LOG IN AS A VENDOR AND GETTOKEN'
//GlobalVariable.katalonVendorEmail = '54katalon@mail.com'
//GlobalVariable.katalonVendorUserId = 'efaea307-58b7-428d-aa1a-8f2ba30bfe16'
def login = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/Authorization/AuthorizationTokenNew',[
	('username') : GlobalVariable.katalonVendorEmail,
	('password') : GlobalVariable.katalonVendorPassword
	]))
GlobalVariable.token = WS.getElementPropertyValue(login, 'token_type') + ' ' + WS.getElementPropertyValue(login, 'access_token') 
	

'VERIFY THE USER IS LOGGED'
loginVerify = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Platform/UserGetCurrentuser(tokenAuthorized)',[
	('token') : GlobalVariable.token
	]))
WS.verifyElementPropertyValue(loginVerify,'userName', GlobalVariable.katalonVendorEmail)
WS.verifyElementPropertyValue(loginVerify,'id', GlobalVariable.katalonVendorUserId)
