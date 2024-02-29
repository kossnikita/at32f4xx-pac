#[doc = "Register `USD` reader"]
pub type R = crate::R<USD_SPEC>;
#[doc = "Field `USDERR` reader - User system data error"]
pub type USDERR_R = crate::BitReader;
#[doc = "Field `FAP` reader - FLASH access protection"]
pub type FAP_R = crate::BitReader;
#[doc = "Field `nWDT_ATO_EN` reader - WDT auto enable"]
pub type N_WDT_ATO_EN_R = crate::BitReader;
#[doc = "Field `nDEPSLP_RST` reader - Deepsleep reset"]
pub type N_DEPSLP_RST_R = crate::BitReader;
#[doc = "Field `nSTDBY_RST` reader - Standby reset"]
pub type N_STDBY_RST_R = crate::BitReader;
#[doc = "Field `BTOPT` reader - boot option"]
pub type BTOPT_R = crate::BitReader;
#[doc = "Field `nWDT_DEPSLP` reader - WDT deep sleep"]
pub type N_WDT_DEPSLP_R = crate::BitReader;
#[doc = "Field `nWDT_STDBY` reader - WDT standby"]
pub type N_WDT_STDBY_R = crate::BitReader;
#[doc = "Field `USER_D0` reader - User data 0"]
pub type USER_D0_R = crate::FieldReader;
#[doc = "Field `USER_D1` reader - User data 1"]
pub type USER_D1_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - User system data error"]
    #[inline(always)]
    pub fn usderr(&self) -> USDERR_R {
        USDERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLASH access protection"]
    #[inline(always)]
    pub fn fap(&self) -> FAP_R {
        FAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDT auto enable"]
    #[inline(always)]
    pub fn n_wdt_ato_en(&self) -> N_WDT_ATO_EN_R {
        N_WDT_ATO_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Deepsleep reset"]
    #[inline(always)]
    pub fn n_depslp_rst(&self) -> N_DEPSLP_RST_R {
        N_DEPSLP_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Standby reset"]
    #[inline(always)]
    pub fn n_stdby_rst(&self) -> N_STDBY_RST_R {
        N_STDBY_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - boot option"]
    #[inline(always)]
    pub fn btopt(&self) -> BTOPT_R {
        BTOPT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - WDT deep sleep"]
    #[inline(always)]
    pub fn n_wdt_depslp(&self) -> N_WDT_DEPSLP_R {
        N_WDT_DEPSLP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WDT standby"]
    #[inline(always)]
    pub fn n_wdt_stdby(&self) -> N_WDT_STDBY_R {
        N_WDT_STDBY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:17 - User data 0"]
    #[inline(always)]
    pub fn user_d0(&self) -> USER_D0_R {
        USER_D0_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - User data 1"]
    #[inline(always)]
    pub fn user_d1(&self) -> USER_D1_R {
        USER_D1_R::new(((self.bits >> 18) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USD")
            .field("usderr", &format_args!("{}", self.usderr().bit()))
            .field("fap", &format_args!("{}", self.fap().bit()))
            .field(
                "n_wdt_ato_en",
                &format_args!("{}", self.n_wdt_ato_en().bit()),
            )
            .field(
                "n_depslp_rst",
                &format_args!("{}", self.n_depslp_rst().bit()),
            )
            .field("n_stdby_rst", &format_args!("{}", self.n_stdby_rst().bit()))
            .field("btopt", &format_args!("{}", self.btopt().bit()))
            .field(
                "n_wdt_depslp",
                &format_args!("{}", self.n_wdt_depslp().bit()),
            )
            .field("n_wdt_stdby", &format_args!("{}", self.n_wdt_stdby().bit()))
            .field("user_d0", &format_args!("{}", self.user_d0().bits()))
            .field("user_d1", &format_args!("{}", self.user_d1().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<USD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "User system data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USD_SPEC;
impl crate::RegisterSpec for USD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usd::R`](R) reader structure"]
impl crate::Readable for USD_SPEC {}
#[doc = "`reset()` method sets USD to value 0x03ff_fffc"]
impl crate::Resettable for USD_SPEC {
    const RESET_VALUE: u32 = 0x03ff_fffc;
}
