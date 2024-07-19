package auth;

import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import org.junit.Assert

/**
 * Class AuthLogin is responsible for handling authentication related operations.
 */
class AuthLogin {
	private String url = 'https://qa-opus.omniapartners.com/sign-in?ReturnUrl=/'
	private String email = 'test549854545@test.com'
	private String password = '1234567Test!'

	/**
	 * Constructor for AuthLogin.
	 */
	AuthLogin() {
	}

	/**
	 * Opens a browser and navigates to the home page.
	 * Reports the navigation action.
	 */
	void visitHomePage() {
		WebUI.openBrowser('')
		WebUI.navigateToUrl(this.url)
		WebUI.comment("Visited ${this.url}")

		// Asserting if the navigation to the URL was successful.
		// Use WebUI.getUrl() to get the current URL and compare it with the expected URL.
		String currentUrl = WebUI.getUrl()
		Assert.assertEquals("The current URL is not as expected.", this.url, currentUrl)
		WebUI.comment("Successfully navigated to the home page.")
	}

	/**
	 * Returns the credentials for the login.
	 * @return Map of credentials containing email and password.
	 */
	Map<String, String> getCredentials() {
		return [email: this.email, password: this.password]
	}

	/**
	 * Check if the email input field is accessible.
	 */
	void checkEmailFieldAccessibility() {
		// Use WebUI.verifyElementVisible to check if the email input field is visible.
		boolean isEmailFieldVisible = WebUI.verifyElementVisible(findTestObject('gonnaputitlatter'), FailureHandling.OPTIONAL)
		Assert.assertTrue("Email input field is not visible.", isEmailFieldVisible)
	}

	/**
	 * Check if the password input field is accessible.
	 */
	void checkPasswordFieldAccessibility() {
		// Use WebUI.verifyElementVisible to check if the password input field is visible.
		boolean isPasswordFieldVisible = WebUI.verifyElementVisible(findTestObject('gonnaputitlatter'), FailureHandling.OPTIONAL)
		Assert.assertTrue("Password input field is not visible.", isPasswordFieldVisible)
	}

	/**
	 * Check if the login button is accessible.
	 */
	void checkLoginButtonAccessibility() {
		// Use WebUI.verifyElementVisible to check if the login button is visible.
		boolean isLoginButtonVisible = WebUI.verifyElementVisible(findTestObject('gonnaputitlatter'), FailureHandling.OPTIONAL)
		Assert.assertTrue("Login button is not visible.", isLoginButtonVisible)
	}

	// Add other methods as needed
}
