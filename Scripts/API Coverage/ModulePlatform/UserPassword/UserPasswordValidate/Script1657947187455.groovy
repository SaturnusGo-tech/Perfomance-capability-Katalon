import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable as GlobalVariable

WebUI.comment('TEST CASE: password validation check')


'PREPARE A MAP OF PASSWORD PARAMETERS'
HashMap<String, String> passwordsMap = GlobalVariable.passwordsListContent


'CRETE LOOP TO VALIDATE PAIRS OF RELATED PASSWORDS/ERROR-MESSAGES BY LOOP ITERATIONS'
for (String password : passwordsMap.keySet()) {
	validatePassword = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/UserPassword/UserPasswordValidate', [
		('password') : password
		]))
	WS.containsString(validatePassword, passwordsMap.get(password), false)
	WebUI.comment('Password: ' + password + ' and the related message: "' + passwordsMap.get(password) + '" are validated')
}


'REPEAT THE SAME OPERATION, BUT THIS TIME FOR ANOTHER VALIDATION ENDPOINT'
for (String password : passwordsMap.keySet()) {
	validatePassword = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/UserPassword/UserPasswordValidateUser', [
		('newPassword') : password,
		('userName') : GlobalVariable.userName
		]))
	WS.containsString(validatePassword, passwordsMap.get(password), false)
	WebUI.comment('Password: ' + password + ' and the related message: "' + passwordsMap.get(password) + '" are validated')
}

