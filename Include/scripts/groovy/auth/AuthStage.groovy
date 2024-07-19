package auth

import auth.AuthLogin
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import org.junit.Assert

/**
 * Class AuthificationStage is responsible for executing the authentication test.
 */
class AuthificationStage {

	/**
	 * Executes the authentication test.
	 */
	void testAuth() {
		// Initialize the AuthLogin object and visit the home page
		AuthLogin authLogin = new AuthLogin()
		authLogin.visitHomePage()

		// Retrieve the credentials
		Map<String, String> credentials = authLogin.getCredentials()
		String email = credentials.get('email')
		String password = credentials.get('password')

		// Wait for the email input element to be present and set the email
		WebUI.waitForElementPresent(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/AuthStage/' + LoginPageLocators.EMAIL_INPUT), 30)
		WebUI.setText(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/AuthStage/' + LoginPageLocators.EMAIL_INPUT), email)

		// Assert that the email was set correctly
		Assert.assertEquals("Email input value is not as expected.", email, WebUI.getAttribute(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/AuthStage/' + LoginPageLocators.EMAIL_INPUT), 'value'))

		// Wait for the password input element to be present and set the password
		WebUI.waitForElementPresent(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/AuthStage/' + LoginPageLocators.PASSWORD_INPUT), 10)
		WebUI.setText(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/AuthStage/' + LoginPageLocators.PASSWORD_INPUT), password)

		// Assert that the password was set correctly
		Assert.assertEquals("Password input value is not as expected.", password, WebUI.getAttribute(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/AuthStage/' + LoginPageLocators.PASSWORD_INPUT), 'value'))

		// Wait for the login button to be present and click it
		WebUI.waitForElementPresent(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/AuthStage/' + LoginPageLocators.LOGIN_BUTTON), 10)
		WebUI.click(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/AuthStage/' + LoginPageLocators.LOGIN_BUTTON))

		// Wait for the element indicating that the user is logged in
		boolean isLoggedIn = WebUI.waitForElementPresent(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/AuthStage/' + LoginPageLocators.LOGGED_IN_ELEMENT), 10)

		// Assert that the user is logged in
		Assert.assertTrue("User is not logged in as expected.", isLoggedIn)
	}
}
