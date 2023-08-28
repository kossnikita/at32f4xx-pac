#[doc = "Register `MACFRMF` reader"]
pub type R = crate::R<MACFRMF_SPEC>;
#[doc = "Register `MACFRMF` writer"]
pub type W = crate::W<MACFRMF_SPEC>;
#[doc = "Field `PR` reader - Promiscuous mode"]
pub type PR_R = crate::BitReader;
#[doc = "Field `PR` writer - Promiscuous mode"]
pub type PR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HUC` reader - Hash unicast"]
pub type HUC_R = crate::BitReader;
#[doc = "Field `HUC` writer - Hash unicast"]
pub type HUC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HMC` reader - Hash multicast"]
pub type HMC_R = crate::BitReader;
#[doc = "Field `HMC` writer - Hash multicast"]
pub type HMC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAIF` reader - Destination address inverse filtering"]
pub type DAIF_R = crate::BitReader;
#[doc = "Field `DAIF` writer - Destination address inverse filtering"]
pub type DAIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PMC` reader - Pass multicast"]
pub type PMC_R = crate::BitReader;
#[doc = "Field `PMC` writer - Pass multicast"]
pub type PMC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBF` reader - Disable broadcast frames"]
pub type DBF_R = crate::BitReader;
#[doc = "Field `DBF` writer - Disable broadcast frames"]
pub type DBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCF` reader - Pass control frames"]
pub type PCF_R = crate::FieldReader;
#[doc = "Field `PCF` writer - Pass control frames"]
pub type PCF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SAIF` reader - Source address inverse filtering"]
pub type SAIF_R = crate::BitReader;
#[doc = "Field `SAIF` writer - Source address inverse filtering"]
pub type SAIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAF` reader - Source address filter"]
pub type SAF_R = crate::BitReader;
#[doc = "Field `SAF` writer - Source address filter"]
pub type SAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPF` reader - Hash or perfect filter"]
pub type HPF_R = crate::BitReader;
#[doc = "Field `HPF` writer - Hash or perfect filter"]
pub type HPF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RA` reader - Receive all"]
pub type RA_R = crate::BitReader;
#[doc = "Field `RA` writer - Receive all"]
pub type RA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass multicast"]
    #[inline(always)]
    pub fn pmc(&self) -> PMC_R {
        PMC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable broadcast frames"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<MACFRMF_SPEC, 0> {
        PR_W::new(self)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    #[must_use]
    pub fn huc(&mut self) -> HUC_W<MACFRMF_SPEC, 1> {
        HUC_W::new(self)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HMC_W<MACFRMF_SPEC, 2> {
        HMC_W::new(self)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DAIF_W<MACFRMF_SPEC, 3> {
        DAIF_W::new(self)
    }
    #[doc = "Bit 4 - Pass multicast"]
    #[inline(always)]
    #[must_use]
    pub fn pmc(&mut self) -> PMC_W<MACFRMF_SPEC, 4> {
        PMC_W::new(self)
    }
    #[doc = "Bit 5 - Disable broadcast frames"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<MACFRMF_SPEC, 5> {
        DBF_W::new(self)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<MACFRMF_SPEC, 6> {
        PCF_W::new(self)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SAIF_W<MACFRMF_SPEC, 8> {
        SAIF_W::new(self)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SAF_W<MACFRMF_SPEC, 9> {
        SAF_W::new(self)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HPF_W<MACFRMF_SPEC, 10> {
        HPF_W::new(self)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<MACFRMF_SPEC, 31> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macfrmf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macfrmf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACFRMF_SPEC;
impl crate::RegisterSpec for MACFRMF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macfrmf::R`](R) reader structure"]
impl crate::Readable for MACFRMF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macfrmf::W`](W) writer structure"]
impl crate::Writable for MACFRMF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACFRMF to value 0"]
impl crate::Resettable for MACFRMF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
