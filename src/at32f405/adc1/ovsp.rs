#[doc = "Register `OVSP` reader"]
pub type R = crate::R<OVSP_SPEC>;
#[doc = "Register `OVSP` writer"]
pub type W = crate::W<OVSP_SPEC>;
#[doc = "Field `OOSEN` reader - Ordinary oversampling enable"]
pub type OOSEN_R = crate::BitReader;
#[doc = "Field `OOSEN` writer - Ordinary oversampling enable"]
pub type OOSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POSEN` reader - Preempted oversampling enable"]
pub type POSEN_R = crate::BitReader;
#[doc = "Field `POSEN` writer - Preempted oversampling enable"]
pub type POSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSRSEL` reader - Oversampling ratio select"]
pub type OSRSEL_R = crate::FieldReader;
#[doc = "Field `OSRSEL` writer - Oversampling ratio select"]
pub type OSRSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `OSSSEL` reader - Oversampling shift select"]
pub type OSSSEL_R = crate::FieldReader;
#[doc = "Field `OSSSEL` writer - Oversampling shift select"]
pub type OSSSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `OOSTREN` reader - Ordinary oversampling trigger mode enable"]
pub type OOSTREN_R = crate::BitReader;
#[doc = "Field `OOSTREN` writer - Ordinary oversampling trigger mode enable"]
pub type OOSTREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOSRSEL` reader - Ordinary oversampling recovery mode select"]
pub type OOSRSEL_R = crate::BitReader;
#[doc = "Field `OOSRSEL` writer - Ordinary oversampling recovery mode select"]
pub type OOSRSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Ordinary oversampling enable"]
    #[inline(always)]
    pub fn oosen(&self) -> OOSEN_R {
        OOSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Preempted oversampling enable"]
    #[inline(always)]
    pub fn posen(&self) -> POSEN_R {
        POSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio select"]
    #[inline(always)]
    pub fn osrsel(&self) -> OSRSEL_R {
        OSRSEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift select"]
    #[inline(always)]
    pub fn osssel(&self) -> OSSSEL_R {
        OSSSEL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Ordinary oversampling trigger mode enable"]
    #[inline(always)]
    pub fn oostren(&self) -> OOSTREN_R {
        OOSTREN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Ordinary oversampling recovery mode select"]
    #[inline(always)]
    pub fn oosrsel(&self) -> OOSRSEL_R {
        OOSRSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OVSP")
            .field("oosrsel", &format_args!("{}", self.oosrsel().bit()))
            .field("oostren", &format_args!("{}", self.oostren().bit()))
            .field("osssel", &format_args!("{}", self.osssel().bits()))
            .field("osrsel", &format_args!("{}", self.osrsel().bits()))
            .field("posen", &format_args!("{}", self.posen().bit()))
            .field("oosen", &format_args!("{}", self.oosen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OVSP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Ordinary oversampling enable"]
    #[inline(always)]
    #[must_use]
    pub fn oosen(&mut self) -> OOSEN_W<OVSP_SPEC, 0> {
        OOSEN_W::new(self)
    }
    #[doc = "Bit 1 - Preempted oversampling enable"]
    #[inline(always)]
    #[must_use]
    pub fn posen(&mut self) -> POSEN_W<OVSP_SPEC, 1> {
        POSEN_W::new(self)
    }
    #[doc = "Bits 2:4 - Oversampling ratio select"]
    #[inline(always)]
    #[must_use]
    pub fn osrsel(&mut self) -> OSRSEL_W<OVSP_SPEC, 2> {
        OSRSEL_W::new(self)
    }
    #[doc = "Bits 5:8 - Oversampling shift select"]
    #[inline(always)]
    #[must_use]
    pub fn osssel(&mut self) -> OSSSEL_W<OVSP_SPEC, 5> {
        OSSSEL_W::new(self)
    }
    #[doc = "Bit 9 - Ordinary oversampling trigger mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn oostren(&mut self) -> OOSTREN_W<OVSP_SPEC, 9> {
        OOSTREN_W::new(self)
    }
    #[doc = "Bit 10 - Ordinary oversampling recovery mode select"]
    #[inline(always)]
    #[must_use]
    pub fn oosrsel(&mut self) -> OOSRSEL_W<OVSP_SPEC, 10> {
        OOSRSEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "oversampling register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovsp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovsp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVSP_SPEC;
impl crate::RegisterSpec for OVSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovsp::R`](R) reader structure"]
impl crate::Readable for OVSP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ovsp::W`](W) writer structure"]
impl crate::Writable for OVSP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OVSP to value 0"]
impl crate::Resettable for OVSP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
